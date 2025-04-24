use crowlink::clients::crownest::{self, *};
use spacetimedb_sdk::{DbContext, Event, Status};

use crate::{
	common::clients::telegram_bot_client::{self, *},
	entities::user_model,
};

pub fn print_message(crowctx: &impl crownest::RemoteDbContext, message: &crownest::Message) {
	let sender = crowctx
		.db()
		.user()
		.identity()
		.find(&message.sender.clone())
		.map(|u| user_model::user_name_or_identity(&u))
		.unwrap_or_else(|| "unknown".to_string());

	println!("{}: {}", sender, message.text);
}

/// Prints new messages.
pub fn on_message_inserted(crowctx: &crownest::EventContext, message: &crownest::Message) {
	if let Event::Reducer(_) = crowctx.event {
		print_message(crowctx, message)
	}
}

/// Prints a warning if the reducer failed.
pub fn on_message_sent(crowctx: &crownest::ReducerEventContext, text: &String) {
	if let Status::Failed(err) = &crowctx.event.status {
		eprintln!("Failed to send message {:?}: {}", text, err);
	}
}

/// Forwards message to Telegram.
pub fn handle_telegram_forward(
	tg_bot: telegram_bot_client::Bot,
) -> impl FnMut(&crownest::EventContext, &crownest::Message) {
	return move |crowctx: &crownest::EventContext, message: &crownest::Message| {
		let tg_bot = tg_bot.clone();

		let sender_name = crowctx
			.db()
			.user()
			.identity()
			.find(&message.sender.clone())
			.map(|u| user_model::user_name_or_identity(&u))
			.unwrap_or_else(|| "unknown".to_string());

		let text = message.text.clone();

		tokio::spawn(async move {
			let _ = tg_bot
				.send_message(
					telegram_bot_client::ChatId(-1001544271932),
					format!("@{}: {}", sender_name, text),
				)
				.await;
		});
	};
}
