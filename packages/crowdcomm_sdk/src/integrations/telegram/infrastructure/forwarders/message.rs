use std::{str::FromStr, sync::Arc};

use corvidx_client::{
	common::stdb::{ActorId, EventContext, ExternalActorReference, Message},
	domain::entities::external_platform::SupportedExternalActorOrigin,
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
		let ext_actor_origin = match &msg.author {
			| ActorId::External(account_id) => ExternalActorReference::from_str(&account_id)
				.map_or(None, |ext_ref| Some(ext_ref.origin.into_supported())),

			| ActorId::Internal(_) | ActorId::Unknown => None,
		};

		// Ignore messages originated from Telegram
		if ext_actor_origin.is_none()
			|| ext_actor_origin.is_some_and(|o| o != SupportedExternalActorOrigin::Telegram)
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
