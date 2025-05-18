use std::sync::Arc;

use crowdcomm_sdk::{
	corvidx::stdb::{DbConnection, MessageTableAccess},
	integrations::telegram::{OutboundTelegramMessage, TelegramForwarder},
	runtime::AsyncHandler,
};
use spacetimedb_sdk::Table;
use teloxide::prelude::Requester;
use tokio::sync::mpsc;

use crate::BotInstanceType;

// TODO: Subscribe to account link requests and forward them to Telegram
// TODO: with accept / decline buttons
/// Sets up account link request forwarding from corvidx to Telegram
/// through a Tokio channel.
pub fn subscribe(
	corvidx: &DbConnection, async_handler: Arc<AsyncHandler>, telegram_bot: BotInstanceType,
) {
	let (tx, mut rx) = mpsc::channel::<OutboundTelegramMessage>(100);
	let bridge = telegram_bot.clone();

	// Spawning a background task that processes messages from the channel
	async_handler.handle().spawn(async move {
		while let Some(msg) = rx.recv().await {
			let _ = bridge
				.send_message(msg.chat_id, &msg.text)
				.await
				.map_err(|err| eprintln!("{:?}", err));
		}
	});

	let forwarder = TelegramForwarder::new(tx, async_handler);

	// Registering the message handler
	corvidx
		.db
		.message()
		.on_insert(move |ctx, msg| forwarder.handle(ctx, msg));
}
