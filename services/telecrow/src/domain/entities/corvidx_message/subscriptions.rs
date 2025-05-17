use std::{str::FromStr, sync::Arc};

use crowdcomm_sdk::{
	corvidx::{
		PlatformAssociation,
		ports::{ProfileResolution, RecordResolution},
		presentation::Displayable,
		stdb::{
			DbConnection, EventContext, ForeignAccountReference, Message, MessageAuthorId,
			ReducerEventContext, send_message,
		},
	},
	integrations::telegram::OutboundTelegramMessage,
};
use spacetimedb_sdk::{Status, Timestamp};
use teloxide::types::Message as TelegramMessage;
use tokio::sync::mpsc;

use crate::common::{constants::TARGET_FOREIGN_PLATFORM_TAG, runtime::AsyncHandler};

/// Forwards message to Telegram using a channel.
pub fn handle_telegram_forward(
	transmitter: mpsc::Sender<OutboundTelegramMessage>, async_handler: Arc<AsyncHandler>,
) -> impl FnMut(&EventContext, &Message) {
	let subscribed_at = Timestamp::now();
	let handle = async_handler.handle();

	return move |corvidx: &EventContext, message: &Message| {
		let foreign_platform_tag = match &message.author_id {
			| MessageAuthorId::ForeignAccountId(account_id) => {
				ForeignAccountReference::from_str(&account_id)
					.map_or(None, |far| Some(far.platform_tag.into_supported()))
			},

			| MessageAuthorId::NativeAccountId(_) | MessageAuthorId::Unknown => None,
		};

		// Ignore messages originated from Telegram
		if foreign_platform_tag.is_none()
			|| foreign_platform_tag.is_some_and(|tag| tag != TARGET_FOREIGN_PLATFORM_TAG)
		{
			// Only forward messages sent after handler initialization
			if subscribed_at.le(&message.sent_at) {
				let author_profile = match &message.author_id {
					| MessageAuthorId::ForeignAccountId(account_id) => account_id
						.resolve(corvidx)
						.map_or(None, |account| account.profile(corvidx)),

					| MessageAuthorId::NativeAccountId(account_id) => account_id
						.resolve(corvidx)
						.map(|native_account| {
							native_account
								.platform_association(corvidx, TARGET_FOREIGN_PLATFORM_TAG)
								.map_or(native_account.profile(corvidx), |foreign_account| {
									foreign_account.profile(corvidx)
								})
						})
						.unwrap_or_default(),

					| MessageAuthorId::Unknown => None,
				};

				let author_name = author_profile
					.map(|profile| profile.display_name())
					.unwrap_or(format!("unknown-{}", message.sender));

				let dto = OutboundTelegramMessage {
					// TODO: The chat id must be taken from crowdcomm_sdk::ForeignChannel
					chat_id: -1001544271932,
					author_name,
					message_text: message.text.clone(),
				};

				// Use the runtime handle to spawn the async task
				let tx = transmitter.clone();

				handle.spawn(async move {
					let _ = tx.send(dto).await;
				});
			}
		}
	};
}

pub fn on_tg_message_received(corvidx: &DbConnection, msg: TelegramMessage) {
	if let Some(text) = msg.text() {
		corvidx.reducers.send_message(text.to_owned()).unwrap();
	}
}

/// Prints a warning if the reducer failed.
fn on_message_sent(corvidx: &ReducerEventContext, text: &String) {
	if let Status::Failed(err) = &corvidx.event.status {
		eprintln!("Failed to send message {:?}: {}", text, err);
	}
}

pub fn subscribe(corvidx: &DbConnection) {
	corvidx.reducers.on_send_message(on_message_sent);
}
