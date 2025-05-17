use std::sync::Arc;

use crowdcomm_sdk::{
	corvidx::stdb::{DbConnection, MessageTableAccess},
	integrations::telegram::OutboundTelegramMessage,
};
use spacetimedb_sdk::Table;
use teloxide::{
	payloads::SendMessageSetters,
	prelude::Requester,
	types::{ChatId, MessageId, ThreadId},
};
use tokio::sync::mpsc;

use crate::{BotInstanceType, common::runtime::AsyncHandler, domain::entities::corvidx_message};

/// Sets up message forwarding from corvidx to Telegram through a Tokio channel.
pub fn subscribe(
	corvidx: &DbConnection, async_handler: Arc<AsyncHandler>, telegram_bot: BotInstanceType,
) {
	let (forward_transmitter, mut forward_receiver) = mpsc::channel::<OutboundTelegramMessage>(100);
	let bridge = telegram_bot.clone();

	// Spawning a background task that processes messages from the channel
	async_handler.handle().spawn(async move {
		while let Some(req) = forward_receiver.recv().await {
			let message_header = format!("💬 <strong>{}</strong>\n\n", req.author_name);
			let message_text = format!("{}{}", message_header, req.message_text);

			let _ = bridge
				.send_message(ChatId(req.chat_id), &message_text)
				.message_thread_id(ThreadId(MessageId(3315)))
				.await
				.map_err(|err| eprintln!("{:?}", err));
		}
	});

	// Registering the message handler
	corvidx
		.db
		.message()
		.on_insert(corvidx_message::handle_telegram_forward(
			forward_transmitter,
			async_handler,
		));
}
