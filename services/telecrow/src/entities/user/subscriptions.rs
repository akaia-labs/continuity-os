use super::user_model;

use std::{sync::Arc, time::Duration};

use crowtocol_rs::crowchat::{self, *};
use spacetimedb_sdk::{DbContext, Status, Timestamp};
use tokio::sync::mpsc;

#[allow(unused_imports)]
use crate::common::{async_runtime::AsyncRuntime, bindings::telegram, runtime::*};

/// If the user is online, prints a notification.
pub fn on_user_inserted(_ctx: &crowchat::EventContext, user: &crowchat::User) {
	if user.is_online {
		println!(
			"User {} connected.",
			user_model::user_name_or_identity(user)
		);
	}
}

/// Prints a notification about name and status changes.
pub fn on_user_updated(_ctx: &crowchat::EventContext, old: &crowchat::User, new: &crowchat::User) {
	if old.name != new.name {
		println!(
			"User {} renamed to {}.",
			user_model::user_name_or_identity(old),
			user_model::user_name_or_identity(new)
		);
	}

	if old.is_online && !new.is_online {
		println!(
			"User {} disconnected.",
			user_model::user_name_or_identity(new)
		);
	}

	if !old.is_online && new.is_online {
		println!("User {} connected.", user_model::user_name_or_identity(new));
	}
}

/// Prints a warning if the reducer failed.
pub fn on_name_set(ctx: &crowchat::ReducerEventContext, name: &String) {
	if let Status::Failed(err) = &ctx.event.status {
		eprintln!("Failed to change name to {:?}: {}", name, err);
	}
}

pub struct StatusTelegramForwardRequest {
	pub chat_id: i64,
	pub sender_name: String,
	pub message_text: String,
}

/// Logs event on Telegram using a channel.
pub fn handle_user_status_telegram_sync(
	transmitter: mpsc::Sender<StatusTelegramForwardRequest>, async_handler: Arc<AsyncRuntime>,
) -> impl FnMut(&crowchat::EventContext, &crowchat::User, &crowchat::User) {
	let subscribed_at = Timestamp::now();
	let handle = async_handler.handle();

	return move |_crowctx: &crowchat::EventContext,
	             outdated_user_data: &crowchat::User,
	             updated_user_data: &crowchat::User| {
		// Only forward events registered after handler initialization
		if subscribed_at.le(&updated_user_data.updated_at) {
			let request = StatusTelegramForwardRequest {
				// TODO: The chat id must be taken from the crowchat room properties
				chat_id: -1001544271932,
				sender_name: "system".to_string(),

				message_text: format!(
					"User {} changed name to {}",
					user_model::user_name_or_identity(outdated_user_data),
					user_model::user_name_or_identity(updated_user_data)
				),
			};

			// Use the runtime handle to spawn the async task
			let tx = transmitter.clone();

			handle.spawn(async move {
				let _ = tx.send(request).await;
			});
		}
	};
}
