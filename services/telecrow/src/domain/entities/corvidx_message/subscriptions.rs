use std::{str::FromStr, sync::Arc};

use crowdcomm_sdk::{
	corvidx::{
		PlatformAssociation,
		ports::{ProfileResolution, RecordResolution},
		presentation::Displayable,
		stdb::{
			DbConnection, EventContext, Message, MessageAuthorId, ReducerEventContext,
			TpAccountReference, send_message,
		},
	},
	integrations::{MessageConverter, telegram::OutboundTelegramMessage},
};
use spacetimedb_sdk::{Status, Timestamp};
use teloxide::types::Message as TelegramMessage;
use tokio::sync::mpsc;

use crate::common::{constants::TARGET_FOREIGN_PLATFORM_TAG, runtime::AsyncHandler};

/// Forwards message to Telegram using a channel.
pub fn handle_telegram_forward(
	tx: mpsc::Sender<OutboundTelegramMessage>, async_handler: Arc<AsyncHandler>,
) -> impl FnMut(&EventContext, &Message) {
	let subscribed_at = Timestamp::now();
	let handle = async_handler.handle();

	return move |corvidx: &EventContext, msg: &Message| {
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
			if subscribed_at.le(&msg.sent_at) {
				let transmitter = tx.clone();

				let dto: OutboundTelegramMessage = TelegramMessage::from_corvidx_message(
					corvidx,
					msg,
					TARGET_FOREIGN_PLATFORM_TAG,
				);

				handle.spawn(async move {
					let result = transmitter.send(dto).await;

					if let Err(err) = result {
						eprintln!("Failed to forward message: {err}");
					}
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
