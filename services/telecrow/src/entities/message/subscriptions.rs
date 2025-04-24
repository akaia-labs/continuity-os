use crowlink::clients::crownest::{self, *};
use spacetimedb_sdk::{Event, Status};

use crate::entities::user_model;

pub fn print_message(ctx: &impl crownest::RemoteDbContext, message: &crownest::Message) {
	let sender = ctx
		.db()
		.user()
		.identity()
		.find(&message.sender.clone())
		.map(|u| user_model::user_name_or_identity(&u))
		.unwrap_or_else(|| "unknown".to_string());

	println!("{}: {}", sender, message.text);
}

/// Prints new messages.
pub fn on_message_inserted(ctx: &crownest::EventContext, message: &crownest::Message) {
	if let Event::Reducer(_) = ctx.event {
		print_message(ctx, message)
	}
}

/// Prints a warning if the reducer failed.
pub fn on_message_sent(ctx: &crownest::ReducerEventContext, text: &String) {
	if let Status::Failed(err) = &ctx.event.status {
		eprintln!("Failed to send message {:?}: {}", text, err);
	}
}
