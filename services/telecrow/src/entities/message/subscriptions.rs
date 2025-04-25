use std::time::Duration;

use crowtocol_rs::crowchat::{self, *};
use spacetimedb_sdk::{DbContext, Status, Timestamp};
use tokio::sync::mpsc;

use crate::{common::runtime::RuntimeService, entities::user_model, features::message_forwarding};
use std::sync::Arc;

/// Prints a warning if the reducer failed.
pub fn on_message_sent(crowctx: &crowchat::ReducerEventContext, text: &String) {
	if let Status::Failed(err) = &crowctx.event.status {
		eprintln!("Failed to send message {:?}: {}", text, err);
	}
}

/// Forwards message to Telegram using a channel.
pub fn handle_telegram_forward(
	transmitter: mpsc::Sender<message_forwarding::TelegramForwardRequest>,
	runtime: Arc<RuntimeService>,
) -> impl FnMut(&crowchat::EventContext, &crowchat::Message) {
	let handle = runtime.handle();

	return move |crowctx: &crowchat::EventContext, message: &crowchat::Message| {
		// Ignore messages inserted by the service itself
		if message.sender != crowctx.identity() {
			// Only forward messages that are not older than 5 minutes
			if Timestamp::now()
				.duration_since(message.sent)
				.lt(&Some(Duration::from_secs(5 * 60)))
			{
				let sender_name = crowctx
					.db()
					.user()
					.identity()
					.find(&message.sender.clone())
					.map(|u| user_model::user_name_or_identity(&u))
					.unwrap_or_else(|| "unknown".to_string());

				let request = message_forwarding::TelegramForwardRequest {
					sender_name,
					message_text: message.text.clone(),
					// TODO: The chat id must be derived from the crowchat chat id
					chat_id: -1001544271932,
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
