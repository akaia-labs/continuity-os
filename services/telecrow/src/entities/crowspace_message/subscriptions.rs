use std::sync::Arc;

use crowcomm::crowspace::{self, *};
use spacetimedb_sdk::{DbContext, Status, Timestamp};
use tokio::sync::mpsc;

use crate::{
	common::{bindings::telegram, runtime::AsyncHandler},
	entities::crowspace_account,
};

pub struct TelegramForwardRequest {
	pub chat_id: i64,
	pub sender_name: String,
	pub message_text: String,
}

/// Forwards message to Telegram using a channel.
pub fn handle_telegram_forward(
	transmitter: mpsc::Sender<TelegramForwardRequest>, async_handler: Arc<AsyncHandler>,
) -> impl FnMut(&crowspace::EventContext, &crowspace::Message) {
	let subscribed_at = Timestamp::now();
	let handle = async_handler.handle();

	return move |stdb: &crowspace::EventContext, message: &crowspace::Message| {
		// Ignore messages inserted by the service itself
		if message.sender != stdb.identity() {
			// Only forward messages sent after handler initialization
			if subscribed_at.le(&message.sent_at) {
				let sender_name = stdb
					.db()
					.account()
					.id()
					.find(&message.sender.clone())
					.map(|u| crowspace_account::identifier(&u))
					.unwrap_or_else(|| "unknown".to_string());

				let request = TelegramForwardRequest {
					// TODO: The chat id must be taken from the crowspace::TextChannel room properties
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

pub fn on_tg_message_received(stdb: &crowspace::DbConnection, msg: telegram::Message) {
	if let Some(text) = msg.text() {
		stdb.reducers.send_message(text.to_owned()).unwrap();
	}
}

/// Prints a warning if the reducer failed.
fn on_message_sent(stdb: &crowspace::ReducerEventContext, text: &String) {
	if let Status::Failed(err) = &stdb.event.status {
		eprintln!("Failed to send message {:?}: {}", text, err);
	}
}

pub fn subscribe(stdb: &crowspace::DbConnection) {
	stdb.reducers.on_send_message(on_message_sent);
}
