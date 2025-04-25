use std::time::Duration;

use crowtocol_rs::crowchat::{self, *};
use spacetimedb_sdk::{DbContext, Status, Timestamp};
use tokio::sync::mpsc;

use crate::{TelegramForwardRequest, entities::user_model};

/// Prints a warning if the reducer failed.
pub fn on_message_sent(crowctx: &crowchat::ReducerEventContext, text: &String) {
	if let Status::Failed(err) = &crowctx.event.status {
		eprintln!("Failed to send message {:?}: {}", text, err);
	}
}

/// Forwards message to Telegram using a channel.
pub fn handle_telegram_forward(
	tx: mpsc::Sender<TelegramForwardRequest>,
) -> impl FnMut(&crowchat::EventContext, &crowchat::Message) {
	return move |crowctx: &crowchat::EventContext, message: &crowchat::Message| {
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

			let _ = tx.try_send(TelegramForwardRequest {
				sender_name,
				message_text: message.text.clone(),
				// TODO: The chat id must be derived from the crowchat chat id
				chat_id: -1001544271932,
			});
		}
	};
}
