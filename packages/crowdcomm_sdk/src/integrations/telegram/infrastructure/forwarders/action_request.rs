use std::{str::FromStr, sync::Arc};

use corvidx_client::{
	common::stdb::{AccountLinkRequest, EventContext, TpAccountReference},
	domain::entities::tp_platform::SupportedTpPlatformTag,
};
use spacetimedb_sdk::Timestamp;
use tokio::sync::mpsc;

use crate::{
	integrations::{ports::CorvidxEventHandler, telegram::OutboundTelegramActionRequest},
	runtime::AsyncHandler,
};

/// A reusable forwarder that listens to action requests from corvidx
/// and pushes them into a Telegram bridge message channel.
pub struct TelegramActionRequestForwarder {
	tx:             mpsc::Sender<OutboundTelegramActionRequest>,
	async_handler:  Arc<AsyncHandler>,
	#[allow(dead_code)]
	initialized_at: Timestamp,
}

impl TelegramActionRequestForwarder {
	pub fn new(
		tx: mpsc::Sender<OutboundTelegramActionRequest>, async_handler: Arc<AsyncHandler>,
	) -> Self {
		TelegramActionRequestForwarder {
			tx,
			async_handler,
			initialized_at: Timestamp::now(),
		}
	}
}

impl CorvidxEventHandler<AccountLinkRequest> for TelegramActionRequestForwarder {
	fn handle(&self, ctx: &EventContext, alr: &AccountLinkRequest) {
		let platform_tag = TpAccountReference::from_str(&alr.subject_account_id)
			.map_or(None, |tpar| Some(tpar.platform_tag.into_supported()));

		if platform_tag.is_some_and(|tag| tag == SupportedTpPlatformTag::Telegram) {
			let dto_result = OutboundTelegramActionRequest::from_account_link_request(ctx, alr);

			if let Ok(dto) = dto_result {
				let tx = self.tx.clone();

				self.async_handler.handle().spawn(async move {
					let result = tx.send(dto).await;

					if let Err(err) = result {
						eprintln!("Failed to send account link request: {err}");
					}
				});
			} else {
				let err = dto_result.unwrap_err();
				eprintln!("Failed to format account link request: {err}");
			}
		}
	}
}
