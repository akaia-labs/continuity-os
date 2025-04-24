use crowlink::clients::crownest::{self, *};
use spacetimedb_sdk::{DbContext, Status};
use tokio::sync::mpsc;

use crate::{TelegramForwardRequest, entities::user_model};

/// Prints a warning if the reducer failed.
pub fn on_message_sent(crowctx: &crownest::ReducerEventContext, text: &String) {
	if let Status::Failed(err) = &crowctx.event.status {
		eprintln!("Failed to send message {:?}: {}", text, err);
	}
}

/// Forwards message to Telegram using a channel.
pub fn handle_telegram_forward(
	tx: mpsc::Sender<TelegramForwardRequest>,
) -> impl FnMut(&crownest::EventContext, &crownest::Message) {
	return move |crowctx: &crownest::EventContext, message: &crownest::Message| {
		let sender_name = crowctx
			.db()
			.user()
			.identity()
			.find(&message.sender.clone())
			.map(|u| user_model::user_name_or_identity(&u))
			.unwrap_or_else(|| "unknown".to_string());

		// Create the request
		let request = TelegramForwardRequest {
			sender_name,
			message_text: message.text.clone(),
			chat_id: -1001544271932, // The hardcoded chat ID
		};

		// This try_send won't block and doesn't require async
		let _ = tx.try_send(request);
	};
}
