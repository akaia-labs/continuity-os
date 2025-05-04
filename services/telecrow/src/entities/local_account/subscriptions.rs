use std::sync::Arc;

use crowcomm::crowd_core::{
	DbConnection, EventContext, LocalAccount, LocalAccountTableAccess, ReducerEventContext,
	set_callsign,
};
use spacetimedb_sdk::{Status, Table, Timestamp};
use tokio::sync::mpsc;

use crate::common::runtime::AsyncHandler;

pub struct StatusTelegramForwardRequest {
	pub chat_id:      i64,
	pub sender_name:  String,
	pub message_text: String,
}

/// Logs event on Telegram using a channel.
pub fn handle_status_telegram_forward(
	transmitter: mpsc::Sender<StatusTelegramForwardRequest>, async_handler: Arc<AsyncHandler>,
) -> impl FnMut(&EventContext, &LocalAccount, &LocalAccount) {
	let subscribed_at = Timestamp::now();
	let handle = async_handler.handle();

	return move |_crowspace_ctx: &EventContext,
	             outdated_account_data: &LocalAccount,
	             updated_account_data: &LocalAccount| {
		// Only forward events registered after handler initialization
		if subscribed_at.le(&updated_account_data.updated_at) {
			if outdated_account_data.callsign != updated_account_data.callsign {
				let request = StatusTelegramForwardRequest {
					// TODO: The chat id must be taken from the crowchat room properties
					chat_id:     -1001544271932,
					sender_name: "system".to_string(),

					message_text: format!(
						"Account {} changed callsign from {} to {}",
						outdated_account_data.id,
						outdated_account_data.callsign,
						updated_account_data.callsign,
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
fn _on_account_updated(_ctx: &EventContext, old: &LocalAccount, new: &LocalAccount) {
	if old.is_online && !new.is_online {
		println!("Account {} disconnected.", old.callsign);
	}

	if !old.is_online && new.is_online {
		println!("Account {} connected.", old.callsign);
	}
}

/// If the account is online, prints a notification.
fn on_account_inserted(_ctx: &EventContext, account: &LocalAccount) {
	if account.is_online {
		println!("Account {} connected.", account.callsign);
	}
}

/// Prints a warning if the reducer failed.
fn on_callsign_set(ctx: &ReducerEventContext, callsign: &String) {
	if let Status::Failed(err) = &ctx.event.status {
		eprintln!("Failed to change callsign to {:?}: {}", callsign, err);
	}
}

pub fn subscribe(core_ctx: &DbConnection) {
	core_ctx.db.local_account().on_insert(on_account_inserted);

	core_ctx.reducers.on_set_callsign(on_callsign_set);
}
