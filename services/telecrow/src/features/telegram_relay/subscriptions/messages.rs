use std::sync::Arc;

use crowcomm::crowspace::{self, *};
use spacetimedb_sdk::Table;
use teloxide::{Bot, payloads::SendMessageSetters, prelude::Requester};
use tokio::sync::mpsc;

use crate::{
	common::{bindings::telegram, runtime::AsyncHandler},
	entities::crowspace_message,
};

/// Sets up message forwarding from crowchat to Telegram.
///
/// This function:
/// 1. Creates the channel for forwarding messages
/// 2. Spawns a background task that processes messages from the channel
/// 3. Registers the message handler
pub fn subscribe(
	stdb: &crowspace::DbConnection, async_handler: Arc<AsyncHandler>, telegram_bot: Bot,
) {
	let (forward_transmitter, mut forward_receiver) =
		mpsc::channel::<crowspace_message::TelegramForwardRequest>(100);

	// Telegram bot instance for the background task
	let telegram_transmitter = telegram_bot.clone();

	// Spawning a background task that processes messages from the channel
	async_handler.handle().spawn(async move {
		while let Some(req) = forward_receiver.recv().await {
			let message_header = format!("💬 {}\n\n", req.sender_name);
			let message_text = format!("{}{}", message_header, req.message_text);

			let _ = telegram_transmitter
				.send_message(telegram::ChatId(req.chat_id), &message_text)
				.message_thread_id(telegram::ThreadId(telegram::MessageId(3315)))
				.await
				.map_err(|err| println!("{:?}", err));
		}
	});

	// Registering the message handler
	stdb.db
		.message()
		.on_insert(crowspace_message::handle_telegram_forward(
			forward_transmitter,
			async_handler,
		));
}
