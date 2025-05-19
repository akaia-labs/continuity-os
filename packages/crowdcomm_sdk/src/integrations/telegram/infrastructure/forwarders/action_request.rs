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

/// A reusable forwarder that listens to command requests from corvidx
/// and pushes them into a Telegram bridge message channel.
pub struct TelegramActionRequestForwarder {
	tx:            mpsc::Sender<OutboundTelegramMessage>,
	async_handler: Arc<AsyncHandler>,
}

impl TelegramActionRequestForwarder {
	pub fn new(
		tx: mpsc::Sender<OutboundTelegramMessage>, async_handler: Arc<AsyncHandler>,
	) -> Self {
		TelegramActionRequestForwarder { tx, async_handler }
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
