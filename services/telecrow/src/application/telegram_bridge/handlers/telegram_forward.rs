use std::{str::FromStr, sync::Arc};

use crowdcomm_sdk::{
	corvidx::{
		stdb::{EventContext, Message, MessageAuthorId, TpAccountReference},
		tp_platform::SupportedTpPlatformTag,
	},
	integrations::{MessageConverter, telegram::OutboundTelegramMessage},
};
use spacetimedb_sdk::Timestamp;
use teloxide::types::Message as TelegramMessage;
use tokio::sync::mpsc;

use crate::common::{constants::TARGET_FOREIGN_PLATFORM_TAG, runtime::AsyncHandler};

/// A reusable forwarder that listens to corvidx [`Message`]s
/// and pushes them into a Telegram bridge message channel.
pub struct TelegramForwarder {
	tx:            mpsc::Sender<OutboundTelegramMessage>,
	async_handler: Arc<AsyncHandler>,
	subscribed_at: Timestamp,
}

impl TelegramForwarder {
	pub fn new(
		tx: mpsc::Sender<OutboundTelegramMessage>, async_handler: Arc<AsyncHandler>,
	) -> Self {
		TelegramForwarder {
			tx,
			async_handler,
			subscribed_at: Timestamp::now(),
		}
	}

	pub fn handle(&self, corvidx: &EventContext, msg: &Message) {
		let tx = self.tx.clone();
		let handle = self.async_handler.handle();

		let tp_platform_tag = match &msg.author_id {
			| MessageAuthorId::TpAccountId(account_id) => TpAccountReference::from_str(&account_id)
				.map_or(None, |far| Some(far.platform_tag.into_supported())),

			| MessageAuthorId::NativeAccountId(_) | MessageAuthorId::Unknown => None,
		};

		// Ignore messages originated from Telegram
		if tp_platform_tag.is_none()
			|| tp_platform_tag.is_some_and(|tag| tag != TARGET_FOREIGN_PLATFORM_TAG)
		{
			// Only forward messages sent after handler initialization
			if self.subscribed_at.le(&msg.sent_at) {
				let transmitter = tx.clone();

				let dto: OutboundTelegramMessage =
					TelegramMessage::from_corvidx_message(corvidx, msg);

				handle.spawn(async move {
					let result = transmitter.send(dto).await;

					if let Err(err) = result {
						eprintln!("Failed to forward message: {err}");
					}
				});
			}
		}
	}
}
