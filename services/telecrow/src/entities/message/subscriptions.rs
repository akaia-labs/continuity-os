use std::{sync::Arc, time::Duration};

use crowtocol_rs::crowchat::{self, *};
use spacetimedb_sdk::{DbContext, Status, Timestamp};
use tokio::sync::mpsc;

use crate::{
	common::{async_runtime::AsyncRuntime, bindings::telegram, runtime::*},
	entities::user_model,
};

/// Prints a warning if the reducer failed.
pub fn on_message_sent(crowctx: &crowchat::ReducerEventContext, text: &String) {
	if let Status::Failed(err) = &crowctx.event.status {
		eprintln!("Failed to send message {:?}: {}", text, err);
	}
}

pub fn on_tg_message_received(crowctx: &crowchat::DbConnection, tg_message: telegram::Message) {
	if let Some(text) = tg_message.text() {
		crowctx.reducers.send_message(text.to_owned()).unwrap();
	}
}

pub struct TelegramForwardRequest {
	pub sender_name: String,
	pub message_text: String,
	pub chat_id: i64,
}

/// Forwards message to Telegram using a channel.
pub fn handle_telegram_forward(
	transmitter: mpsc::Sender<TelegramForwardRequest>, runtime: Arc<AsyncRuntime>,
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

				let request = TelegramForwardRequest {
					sender_name,
					message_text: message.text.clone(),
					// TODO: The chat id must be taken from the crowchat room properties
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

pub async fn process_text_message(
	_tg_bot: telegram::Bot, tg_user: telegram::User, message_text: String,
) -> Result<(), TelecrowError> {
	println!(
		"@{}: {}",
		tg_user.username.clone().unwrap_or(tg_user.id.to_string()),
		message_text
	);

	// let _message = tg_bot
	// 	.send_message(
	// 		tg_user.id,
	// 		format!(
	// 			"@{:#?}: {}",
	// 			tg_user.username.unwrap_or(tg_user.id.to_string()),
	// 			message_text
	// 		),
	// 	)
	// 	.await
	// 	.unwrap();

	Ok(())
}
