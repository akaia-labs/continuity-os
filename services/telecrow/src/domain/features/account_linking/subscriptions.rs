use std::sync::Arc;

use crowdcomm_sdk::{
	corvidx::stdb::{AccountLinkRequestTableAccess, DbConnection},
	integrations::{
		CorvidxEventHandler,
		telegram::{OutboundTelegramActionRequest, TelegramActionRequestForwarder},
	},
	runtime::AsyncHandler,
};
use spacetimedb_sdk::Table;
use teloxide::{payloads::SendMessageSetters, prelude::Requester};
use tokio::sync::mpsc;

use crate::BotInstanceType;

/// Sets up account link request forwarding from corvidx to Telegram
/// through a Tokio channel.
pub fn subscribe(
	corvidx: &DbConnection, async_handler: Arc<AsyncHandler>, telegram_bot: BotInstanceType,
) {
	let (tx, mut rx) = mpsc::channel::<OutboundTelegramActionRequest>(100);
	let bridge = telegram_bot.clone();

	// Spawning a background task that processes messages from the channel
	async_handler.handle().spawn(async move {
		while let Some(req) = rx.recv().await {
			let _ = bridge
				.send_message(req.chat_id, &req.text)
				.reply_markup(req.reply_markup)
				.await
				.map_err(|err| eprintln!("{:?}", err));
		}
	});

	let forwarder = TelegramActionRequestForwarder::new(tx, async_handler);

	// Registering the message handler
	corvidx
		.db
		.account_link_request()
		.on_insert(move |ctx, alr| forwarder.handle(ctx, alr));
}
