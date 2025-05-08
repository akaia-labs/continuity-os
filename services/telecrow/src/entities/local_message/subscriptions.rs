use std::sync::Arc;

use crowdcomm::corvidx::{
	DbConnection, EventContext, LocalAccountTableAccess, Message, ReducerEventContext,
	send_message, traits::DisplayName,
};
use spacetimedb_sdk::{DbContext, Status, Timestamp};
use teloxide::types::Message as TelegramMessage;
use tokio::sync::mpsc;

use crate::common::runtime::AsyncHandler;

pub struct TelegramForwardRequest {
	pub chat_id:      i64,
	pub sender_name:  String,
	pub message_text: String,
}

/// Forwards message to Telegram using a channel.
pub fn handle_telegram_forward(
	transmitter: mpsc::Sender<TelegramForwardRequest>, async_handler: Arc<AsyncHandler>,
) -> impl FnMut(&EventContext, &Message) {
	let subscribed_at = Timestamp::now();
	let handle = async_handler.handle();

	return move |corvidx: &EventContext, message: &Message| {
		// Ignore messages inserted by the service itself
		if message.sender != corvidx.identity() {
			// Only forward messages sent after handler initialization
			if subscribed_at.le(&message.sent_at) {
				let sender_name = corvidx
					.db()
					.local_account()
					.id()
					.find(&message.sender.clone())
					.map(|account| account.display_name(corvidx))
					.unwrap_or(format!("{}", message.sender));

				let request = TelegramForwardRequest {
					// TODO: The chat id must be taken from crowdcomm::TextChannel
					chat_id: -1001544271932,
					sender_name,
					message_text: message.text.clone(),
				};

				// Use the runtime handle to spawn the async task
				let tx = transmitter.clone();

				handle.spawn(async move {
					let _ = tx.send(request).await;
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
