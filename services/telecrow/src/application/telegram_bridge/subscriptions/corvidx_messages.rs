use std::sync::Arc;

use crowdcomm_sdk::{
	corvidx::stdb::{DbConnection, MessageTableAccess},
	integrations::telegram::OutboundTelegramMessage,
};
use spacetimedb_sdk::Table;
use teloxide::{
	payloads::SendMessageSetters,
	prelude::Requester,
	types::{MessageId, ThreadId},
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
		while let Some(msg) = forward_receiver.recv().await {
			let message_request = bridge.send_message(msg.chat_id, &msg.text);

			let _ = message_request
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
