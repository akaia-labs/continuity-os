use crowtocol_rs::crowchat::{self, *};
use spacetimedb_sdk::Table;
use std::sync::Arc;
use tokio::sync::mpsc;

use crate::{
	common::{
		bindings::telegram::{self, *},
		async_runtime::AsyncRuntime,
	},
	entities::message_subscriptions,
};

pub struct TelegramForwardRequest {
	pub sender_name: String,
	pub message_text: String,
	pub chat_id: i64,
}

/// Sets up message forwarding from crowchat to Telegram.
///
/// This function:
/// 1. Creates the channel for forwarding messages
/// 2. Spawns a background task that processes messages from the channel
/// 3. Registers the message handler
/// 4. Returns the sender that can be used to send messages to the channel
pub fn start_forwarding(
	telegram_bot: telegram::Bot, runtime: Arc<AsyncRuntime>, crowctx: &crowchat::DbConnection,
) -> mpsc::Sender<TelegramForwardRequest> {
	// Create channel for forwarding messages to Telegram
	let (forward_transmitter, mut forward_receiver) = mpsc::channel::<TelegramForwardRequest>(100);

	// Clone the sender to return it
	let sender_to_return = forward_transmitter.clone();

	// Telegram bot instance for the background task
	let telegram_transmitter = telegram_bot.clone();

	// Spawn a background task that processes messages from the channel
	runtime.handle().spawn(async move {
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
			runtime,
		));

	// Return the sender
	sender_to_return
}
