use std::sync::Arc;

use crowdcomm::crowd_core::{DbConnection, LocalAccountTableAccess};
use spacetimedb_sdk::TableWithPrimaryKey;
use teloxide::{
	payloads::SendMessageSetters,
	prelude::Requester,
	types::{ChatId, MessageId, ThreadId},
};
use tokio::sync::mpsc;

use crate::{BotInstanceType, common::runtime::AsyncHandler, entities::local_account};

/// Sets up event forwarding from crowchat to Telegram.
///
/// This function:
/// 1. Creates the channel for sending event messages
/// 2. Spawns a background task that processes events from the channel
/// 3. Registers the event handler
pub fn subscribe(
	core_ctx: &DbConnection, async_handler: Arc<AsyncHandler>, telegram_bot: BotInstanceType,
) {
	let (forward_transmitter, mut forward_receiver) =
		mpsc::channel::<local_account::StatusTelegramForwardRequest>(100);

	// Telegram bot instance for the background task
	let telegram_transmitter = telegram_bot.clone();

	// Spawning a background task that processes messages from the channel
	async_handler.handle().spawn(async move {
		while let Some(req) = forward_receiver.recv().await {
			let message_header = format!("ℹ️ <strong>{}</strong>\n\n", req.sender_name);
			let message_text = format!("{}{}", message_header, req.message_text);

			let _ = telegram_transmitter
				.send_message(ChatId(req.chat_id), message_text)
				.message_thread_id(ThreadId(MessageId(3315)))
				.await
				.map_err(|err| println!("{:?}", err));
		}
	});

	// Registering the event handler
	core_ctx
		.db
		.local_account()
		.on_update(local_account::handle_status_telegram_forward(
			forward_transmitter,
			async_handler,
		));
}
