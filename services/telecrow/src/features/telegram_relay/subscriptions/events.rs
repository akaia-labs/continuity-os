use crowcomm::crowspace::{self, *};
use spacetimedb_sdk::TableWithPrimaryKey;
use std::sync::Arc;
use teloxide::{Bot, payloads::SendMessageSetters, prelude::Requester};
use tokio::sync::mpsc;

use crate::{
	common::{runtime::AsyncHandler, bindings::telegram},
	entities::crowspace_account,
};

/// Sets up event forwarding from crowchat to Telegram.
///
/// This function:
/// 1. Creates the channel for sending event messages
/// 2. Spawns a background task that processes events from the channel
/// 3. Registers the event handler
pub fn subscribe(
	stdb: &crowspace::DbConnection, async_handler: Arc<AsyncHandler>, telegram_bot: Bot,
) {
	let (forward_transmitter, mut forward_receiver) =
		mpsc::channel::<crowspace_account::StatusTelegramForwardRequest>(100);

	// Telegram bot instance for the background task
	let telegram_transmitter = telegram_bot.clone();

	// Spawning a background task that processes messages from the channel
	async_handler.handle().spawn(async move {
		while let Some(req) = forward_receiver.recv().await {
			let _ = telegram_transmitter
				.send_message(
					telegram::ChatId(req.chat_id),
					format!("ℹ️ {}: {}", req.sender_name, req.message_text),
				)
				.message_thread_id(telegram::ThreadId(telegram::MessageId(3315)))
				.await;
		}
	});

	// Registering the event handler
	stdb.db
		.account()
		.on_update(crowspace_account::handle_status_telegram_forward(
			forward_transmitter,
			async_handler,
		));
}
