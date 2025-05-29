use std::{str::FromStr, sync::Arc};

use corvidx_client::{
	common::stdb::{EventContext, ExternalActorReference, ExternalAuthenticationRequest},
	domain::entities::external_platform::SupportedExternalActorOrigin,
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

impl CorvidxEventHandler<ExternalAuthenticationRequest> for TelegramActionRequestForwarder {
	fn handle(&self, ctx: &EventContext, ext_auth_req: &ExternalAuthenticationRequest) {
		let ext_actor_origin = ExternalActorReference::from_str(&ext_auth_req.subject)
			.map_or(None, |ext_ref| Some(ext_ref.origin.into_supported()));

		if ext_actor_origin.is_some_and(|tag| tag == SupportedExternalActorOrigin::Telegram) {
			let dto_result = OutboundTelegramActionRequest::from_ext_auth_req(ctx, ext_auth_req);

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
