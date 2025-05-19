use std::sync::Arc;

use crowdcomm_sdk::{
	corvidx::stdb::{DbConnection, MessageTableAccess},
	integrations::{
		CorvidxEventHandler,
		telegram::{OutboundTelegramMessage, TelegramMessageForwarder},
	},
	runtime::AsyncHandler,
};
use spacetimedb_sdk::Table;
use teloxide::{payloads::SendMessageSetters, prelude::Requester, sugar::request::RequestReplyExt};
use tokio::sync::mpsc;

use crate::BotInstanceType;

/// Sets up message forwarding from corvidx to Telegram through a Tokio channel.
pub fn subscribe(
	corvidx: &DbConnection, async_handler: Arc<AsyncHandler>, telegram_bot: BotInstanceType,
) {
	let (tx, mut rx) = mpsc::channel::<OutboundTelegramMessage>(100);
	let bridge = telegram_bot.clone();

	// Spawning a background task that processes messages from the channel
	async_handler.handle().spawn(async move {
		while let Some(msg) = rx.recv().await {
			let mut req = bridge.send_message(msg.chat_id, &msg.text);

			if let Some(thread_id) = msg.thread_id {
				req = req.message_thread_id(thread_id)
			};

			if let Some(reply_to_message_id) = msg.reply_to_message_id {
				req = req.reply_to(reply_to_message_id)
			};

			let _ = req.await.map_err(|err| eprintln!("{:?}", err));
		}
	});

	let forwarder = TelegramMessageForwarder::new(tx, async_handler);

	// Registering the message handler
	corvidx
		.db
		.message()
		.on_insert(move |ctx, msg| forwarder.handle(ctx, msg));
}
