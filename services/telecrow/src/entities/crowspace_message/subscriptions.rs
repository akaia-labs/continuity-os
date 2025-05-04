use std::sync::Arc;

use crowcomm::{
	crowd_core::{
		AccountTableAccess, DbConnection, EventContext, Message, ReducerEventContext, send_message,
		traits::DisplayName,
	},
	telegram,
};
use spacetimedb_sdk::{DbContext, Status, Timestamp};
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

	return move |crowspace_ctx: &EventContext, message: &Message| {
		// Ignore messages inserted by the service itself
		if message.sender != crowspace_ctx.identity() {
			// Only forward messages sent after handler initialization
			if subscribed_at.le(&message.sent_at) {
				let sender_name = crowspace_ctx
					.db()
					.account()
					.id()
					.find(&message.sender.clone())
					.map(|account| account.display_name(crowspace_ctx))
					.unwrap_or(format!("{}", message.sender));

				let request = TelegramForwardRequest {
					// TODO: The chat id must be taken from crowspace::TextChannel
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

pub fn on_tg_message_received(crowspace_ctx: &DbConnection, msg: telegram::Message) {
	if let Some(text) = msg.text() {
		crowspace_ctx
			.reducers
			.send_message(text.to_owned())
			.unwrap();
	}
}

/// Prints a warning if the reducer failed.
fn on_message_sent(crowspace_ctx: &ReducerEventContext, text: &String) {
	if let Status::Failed(err) = &crowspace_ctx.event.status {
		eprintln!("Failed to send message {:?}: {}", text, err);
	}
}

pub fn subscribe(crowspace_ctx: &DbConnection) {
	crowspace_ctx.reducers.on_send_message(on_message_sent);
}
