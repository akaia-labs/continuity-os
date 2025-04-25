use crowtocol_rs::crowchat::{self, *};
use spacetimedb_sdk::Table;
use std::sync::Arc;
use tokio::sync::mpsc;

use crate::{
	common::{
		async_runtime::AsyncRuntime,
		bindings::telegram::{self, *},
	},
	entities::message_subscriptions,
};

/// Sets up event forwarding from crowchat to Telegram.
///
/// This function:
/// 1. Creates the channel for sending event messages
/// 2. Spawns a background task that processes events from the channel
/// 3. Registers the message handler
pub fn event_capture_init(
	telegram_bot: telegram::Bot, async_handler: Arc<AsyncRuntime>, crowctx: &crowchat::DbConnection,
) {
	let (forward_transmitter, mut forward_receiver) =
		mpsc::channel::<message_subscriptions::TelegramForwardRequest>(100);

	// Telegram bot instance for the background task
	let telegram_transmitter = telegram_bot.clone();

	// Spawn a background task that processes messages from the channel
	async_handler.handle().spawn(async move {
		while let Some(req) = forward_receiver.recv().await {
			let _ = telegram_transmitter
				.send_message(
					telegram::ChatId(req.chat_id),
					format!("💬 {}: {}", req.sender_name, req.message_text),
				)
				.message_thread_id(telegram::ThreadId(telegram::MessageId(3315)))
				.await;
		}
	});

	// Register the message handler directly
	crowctx
		.db
		.message()
		.on_insert(message_subscriptions::handle_telegram_forward(
			forward_transmitter,
			async_handler,
		));
}
