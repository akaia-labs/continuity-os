use std::{str::FromStr, sync::Arc};

use corvidx_client::{
	common::stdb::{EventContext, Message, MessageAuthorId, TpAccountReference},
	domain::entities::tp_platform::SupportedTpPlatformTag,
};
use spacetimedb_sdk::Timestamp;
use tokio::sync::mpsc;

use crate::{
	integrations::{ports::CorvidxEventHandler, telegram::OutboundTelegramMessage},
	runtime::AsyncHandler,
};

/// A reusable forwarder that listens to corvidx [`Message`]s
/// and pushes them into a Telegram bridge message channel.
pub struct TelegramMessageForwarder {
	tx:             mpsc::Sender<OutboundTelegramMessage>,
	async_handler:  Arc<AsyncHandler>,
	initialized_at: Timestamp,
}

impl TelegramMessageForwarder {
	pub fn new(
		tx: mpsc::Sender<OutboundTelegramMessage>, async_handler: Arc<AsyncHandler>,
	) -> Self {
		TelegramMessageForwarder {
			tx,
			async_handler,
			initialized_at: Timestamp::now(),
		}
	}
}

impl CorvidxEventHandler<Message> for TelegramMessageForwarder {
	fn handle(&self, ctx: &EventContext, msg: &Message) {
		let tp_platform_tag = match &msg.author_id {
			| MessageAuthorId::TpAccountId(account_id) => TpAccountReference::from_str(&account_id)
				.map_or(None, |far| Some(far.platform_tag.into_supported())),

			| MessageAuthorId::NativeAccountId(_) | MessageAuthorId::Unknown => None,
		};

		// Ignore messages originated from Telegram
		if tp_platform_tag.is_none()
			|| tp_platform_tag.is_some_and(|tag| tag != SupportedTpPlatformTag::Telegram)
		{
			// Only forward messages sent after forwarder initialization
			if self.initialized_at.le(&msg.sent_at) {
				let tx = self.tx.clone();
				let dto = OutboundTelegramMessage::from_native(ctx, msg);

				self.async_handler.handle().spawn(async move {
					let result = tx.send(dto).await;

					if let Err(err) = result {
						eprintln!("Failed to forward message: {err}");
					}
				});
			}
		}
	}
}
