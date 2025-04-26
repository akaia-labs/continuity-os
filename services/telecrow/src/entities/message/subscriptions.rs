use std::sync::Arc;

use crowtocol_rs::crowchat::{self, *};
use spacetimedb_sdk::{DbContext, Status, Timestamp};
use tokio::sync::mpsc;

use crate::{
	common::{async_runtime::AsyncRuntime, bindings::telegram, runtime::*},
	entities::user_model,
};

pub struct TelegramForwardRequest {
	pub chat_id: i64,
	pub sender_name: String,
	pub message_text: String,
}

/// Forwards message to Telegram using a channel.
pub fn handle_telegram_forward(
	transmitter: mpsc::Sender<TelegramForwardRequest>, async_handler: Arc<AsyncRuntime>,
) -> impl FnMut(&crowchat::EventContext, &crowchat::Message) {
	let subscribed_at = Timestamp::now();
	let handle = async_handler.handle();

	return move |crowctx: &crowchat::EventContext, message: &crowchat::Message| {
		// Ignore messages inserted by the service itself
		if message.sender != crowctx.identity() {
			// Only forward messages sent after handler initialization
			if subscribed_at.le(&message.sent) {
				let sender_name = crowctx
					.db()
					.user()
					.identity()
					.find(&message.sender.clone())
					.map(|u| user_model::user_name_or_identity(&u))
					.unwrap_or_else(|| "unknown".to_string());

				let request = TelegramForwardRequest {
					// TODO: The chat id must be taken from the crowchat room properties
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

pub fn on_tg_message_received(crowctx: &crowchat::DbConnection, tg_message: telegram::Message) {
	if let Some(text) = tg_message.text() {
		crowctx.reducers.send_message(text.to_owned()).unwrap();
	}
}

/// Prints a warning if the reducer failed.
fn on_message_sent(crowctx: &crowchat::ReducerEventContext, text: &String) {
	if let Status::Failed(err) = &crowctx.event.status {
		eprintln!("Failed to send message {:?}: {}", text, err);
	}
}

pub fn register_internal_callbacks(crowctx: &crowchat::DbConnection) {
	crowctx.reducers.on_send_message(on_message_sent);
}
