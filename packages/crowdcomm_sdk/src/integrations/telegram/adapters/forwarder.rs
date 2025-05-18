use std::{str::FromStr, sync::Arc};

use corvidx_client::{
	common::stdb::{
		AccountLinkRequest, EventContext, Message, MessageAuthorId, TpAccountReference,
	},
	domain::entities::tp_platform::SupportedTpPlatformTag,
};
use spacetimedb_sdk::Timestamp;
use tokio::sync::mpsc;

use crate::{integrations::telegram::OutboundTelegramMessage, runtime::AsyncHandler};

/// A reusable forwarder that listens to corvidx [`Message`]s
/// and pushes them into a Telegram bridge message channel.
pub struct TelegramForwarder {
	tx:             mpsc::Sender<OutboundTelegramMessage>,
	async_handler:  Arc<AsyncHandler>,
	initialized_at: Timestamp,
}

impl TelegramForwarder {
	pub fn new(
		tx: mpsc::Sender<OutboundTelegramMessage>, async_handler: Arc<AsyncHandler>,
	) -> Self {
		TelegramForwarder {
			tx,
			async_handler,
			initialized_at: Timestamp::now(),
		}
	}

	pub fn handle_message(&self, corvidx: &EventContext, msg: &Message) {
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
				let dto = OutboundTelegramMessage::from_native(corvidx, msg);

				self.async_handler.handle().spawn(async move {
					let result = tx.send(dto).await;

					if let Err(err) = result {
						eprintln!("Failed to forward message: {err}");
					}
				});
			}
		}
	}

	pub fn handle_account_link_request(&self, corvidx: &EventContext, alr: &AccountLinkRequest) {
		let platform_tag = TpAccountReference::from_str(&alr.subject_account_id)
			.map_or(None, |tpar| Some(tpar.platform_tag.into_supported()));

		if platform_tag.is_some_and(|tag| tag == SupportedTpPlatformTag::Telegram) {
			let dto_result = OutboundTelegramMessage::from_account_link_request(corvidx, alr);

			if let Ok(dto) = dto_result {
				let tx = self.tx.clone();

				self.async_handler.handle().spawn(async move {
					let result = tx.send(dto).await;

					if let Err(err) = result {
						eprintln!("Failed to send account link request: {err}");
					}
				});
			} else {
				eprintln!("Failed to format account link request: {dto_result:?}");
			}
		}
	}
}
