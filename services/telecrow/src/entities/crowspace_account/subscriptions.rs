use std::sync::Arc;

use crowcomm::crowspace::{self, *};
use spacetimedb_sdk::{Status, Table, Timestamp};
use tokio::sync::mpsc;

use super::model;
use crate::common::runtime::AsyncHandler;

pub struct StatusTelegramForwardRequest {
	pub chat_id:      i64,
	pub sender_name:  String,
	pub message_text: String,
}

/// Logs event on Telegram using a channel.
pub fn handle_status_telegram_forward(
	transmitter: mpsc::Sender<StatusTelegramForwardRequest>, async_handler: Arc<AsyncHandler>,
) -> impl FnMut(&crowspace::EventContext, &crowspace::Account, &crowspace::Account) {
	let subscribed_at = Timestamp::now();
	let handle = async_handler.handle();

	return move |_stdb: &crowspace::EventContext,
	             outdated_account_data: &crowspace::Account,
	             updated_account_data: &crowspace::Account| {
		// Only forward events registered after handler initialization
		if subscribed_at.le(&updated_account_data.updated_at) {
			if outdated_account_data.callsign != updated_account_data.callsign {
				let request = StatusTelegramForwardRequest {
					// TODO: The chat id must be taken from the crowchat room properties
					chat_id:     -1001544271932,
					sender_name: "system".to_string(),

					message_text: format!(
						"Account {} changed callsign to {}",
						model::identifier(outdated_account_data),
						model::identifier(updated_account_data)
					),
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

/// @deprecated
/// Prints a notification about callsign and status changes.
fn _on_account_updated(
	_ctx: &crowspace::EventContext, old: &crowspace::Account, new: &crowspace::Account,
) {
	if old.is_online && !new.is_online {
		println!("Account {} disconnected.", model::identifier(new));
	}

	if !old.is_online && new.is_online {
		println!("Account {} connected.", model::identifier(new));
	}
}

/// If the account is online, prints a notification.
fn on_account_inserted(_ctx: &crowspace::EventContext, account: &crowspace::Account) {
	if account.is_online {
		println!("Account {} connected.", model::identifier(account));
	}
}

/// Prints a warning if the reducer failed.
fn on_callsign_set(ctx: &crowspace::ReducerEventContext, callsign: &String) {
	if let Status::Failed(err) = &ctx.event.status {
		eprintln!("Failed to change callsign to {:?}: {}", callsign, err);
	}
}

pub fn subscribe(stdb: &crowspace::DbConnection) {
	stdb.db.account().on_insert(on_account_inserted);
	stdb.reducers.on_set_callsign(on_callsign_set);
}
