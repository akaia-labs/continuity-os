use std::sync::Arc;

use crowcomm::crowd_core::{DbConnection, MessageTableAccess};
use spacetimedb_sdk::Table;
use teloxide::{
	payloads::SendMessageSetters,
	prelude::Requester,
	types::{ChatId, MessageEntity, MessageEntityKind, MessageId, ThreadId},
};
use tokio::sync::mpsc;

use crate::{BotInstanceType, common::runtime::AsyncHandler, entities::local_message};

/// Sets up message forwarding from crowchat to Telegram.
///
/// This function:
/// 1. Creates the channel for forwarding messages
/// 2. Spawns a background task that processes messages from the channel
/// 3. Registers the message handler
pub fn subscribe(
	core_ctx: &DbConnection, async_handler: Arc<AsyncHandler>, telegram_bot: BotInstanceType,
) {
	let (forward_transmitter, mut forward_receiver) =
		mpsc::channel::<local_message::TelegramForwardRequest>(100);

	// Telegram bot instance for the background task
	let telegram_transmitter = telegram_bot.clone();

	// Spawning a background task that processes messages from the channel
	async_handler.handle().spawn(async move {
		while let Some(req) = forward_receiver.recv().await {
			let message_header = format!("💬 {}\n\n", req.sender_name);
			let message_header_length = message_header.encode_utf16().count();
			let message_text = format!("{}{}", message_header, req.message_text);

			let _ = telegram_transmitter
				.send_message(ChatId(req.chat_id), &message_text)
				.entities([MessageEntity::new(
					MessageEntityKind::Bold,
					0,
					message_header_length,
				)])
				.message_thread_id(ThreadId(MessageId(3315)))
				.await
				.map_err(|err| eprintln!("{:?}", err));
		}
	});

	// Registering the message handler
	core_ctx
		.db
		.message()
		.on_insert(local_message::handle_telegram_forward(
			forward_transmitter,
			async_handler,
		));
}
