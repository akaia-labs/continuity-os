use std::sync::Arc;

use crowdcomm_sdk::{
	corvidx::stdb::{DbConnection, NativeAccountTableAccess},
	integrations::telegram::OutboundTelegramMessage,
};
use spacetimedb_sdk::TableWithPrimaryKey;
use teloxide::{
	payloads::SendMessageSetters,
	prelude::Requester,
	types::{ChatId, MessageId, ThreadId},
};
use tokio::sync::mpsc;

use crate::{BotInstanceType, common::runtime::AsyncHandler, domain::entities::corvidx_account};

/// Sets up event forwarding from corvidx to Telegram through a Tokio channel.
pub fn subscribe(
	corvidx: &DbConnection, async_handler: Arc<AsyncHandler>, telegram_bot: BotInstanceType,
) {
	let (forward_transmitter, mut forward_receiver) = mpsc::channel::<OutboundTelegramMessage>(100);
	let bridge = telegram_bot.clone();

	// Spawning a background task that processes messages from the channel
	async_handler.handle().spawn(async move {
		while let Some(message) = forward_receiver.recv().await {
			let message_header = format!("ℹ️ <strong>{}</strong>\n\n", message.author_name);
			let message_text = format!("{}{}", message_header, message.text);

			let _ = bridge
				.send_message(ChatId(message.chat_id), message_text)
				.message_thread_id(ThreadId(MessageId(3315)))
				.await
				.map_err(|err| println!("{:?}", err));
		}
	});

	// Registering the event handler
	corvidx
		.db
		.native_account()
		.on_update(corvidx_account::handle_status_telegram_forward(
			forward_transmitter,
			async_handler,
		));
}
