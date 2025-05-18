use std::sync::Arc;

use crowdcomm_sdk::{
	corvidx::stdb::{DbConnection, MessageTableAccess},
	integrations::telegram::{OutboundTelegramMessage, TelegramForwarder},
	runtime::AsyncHandler,
};
use spacetimedb_sdk::Table;
use teloxide::{payloads::SendMessageSetters, prelude::Requester};
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
			let basic_request = bridge.send_message(msg.chat_id, &msg.text);

			let final_request = if let Some(thread_id) = msg.thread_id {
				basic_request.message_thread_id(thread_id)
			} else {
				basic_request
			};

			let _ = final_request.await.map_err(|err| eprintln!("{:?}", err));
		}
	});

	let forwarder = TelegramForwarder::new(tx, async_handler);

	// Registering the message handler
	corvidx
		.db
		.message()
		.on_insert(move |ctx, msg| forwarder.handle(ctx, msg));
}
