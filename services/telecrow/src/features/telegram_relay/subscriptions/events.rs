use std::sync::Arc;

use crowcomm::{
	crowd_core::{AccountTableAccess, DbConnection},
	telegram,
};
use spacetimedb_sdk::TableWithPrimaryKey;
use teloxide::{
	Bot,
	payloads::SendMessageSetters,
	prelude::Requester,
	types::{MessageEntity, MessageEntityKind},
};
use tokio::sync::mpsc;

use crate::{common::runtime::AsyncHandler, entities::crowspace_account};

/// Sets up event forwarding from crowchat to Telegram.
///
/// This function:
/// 1. Creates the channel for sending event messages
/// 2. Spawns a background task that processes events from the channel
/// 3. Registers the event handler
pub fn subscribe(
	crowspace_ctx: &DbConnection, async_handler: Arc<AsyncHandler>, telegram_bot: Bot,
) {
	let (forward_transmitter, mut forward_receiver) =
		mpsc::channel::<crowspace_account::StatusTelegramForwardRequest>(100);

	// Telegram bot instance for the background task
	let telegram_transmitter = telegram_bot.clone();

	// Spawning a background task that processes messages from the channel
	async_handler.handle().spawn(async move {
		while let Some(req) = forward_receiver.recv().await {
			let message_header = format!("ℹ️ {}\n\n", req.sender_name);
			let message_header_length = message_header.encode_utf16().count();
			let message_text = format!("{}{}", message_header, req.message_text);

			let _ = telegram_transmitter
				.send_message(telegram::ChatId(req.chat_id), message_text)
				.entities([MessageEntity::new(
					MessageEntityKind::Bold,
					0,
					message_header_length,
				)])
				.message_thread_id(telegram::ThreadId(telegram::MessageId(3315)))
				.await
				.map_err(|err| println!("{:?}", err));
		}
	});

	// Registering the event handler
	crowspace_ctx
		.db
		.account()
		.on_update(crowspace_account::handle_status_telegram_forward(
			forward_transmitter,
			async_handler,
		));
}
