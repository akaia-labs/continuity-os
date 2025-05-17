use std::sync::Arc;

use crowdcomm_sdk::{
	corvidx::stdb::{
		DbConnection, EventContext, NativeAccount, NativeAccountTableAccess, ReducerEventContext,
		set_account_callsign,
	},
	integrations::telegram::OutboundTelegramMessage,
};
use spacetimedb_sdk::{Status, Table, Timestamp};
use tokio::sync::mpsc;

use crate::common::runtime::AsyncHandler;

/// Logs event on Telegram using a channel.
pub fn handle_status_telegram_forward(
	transmitter: mpsc::Sender<OutboundTelegramMessage>, async_handler: Arc<AsyncHandler>,
) -> impl FnMut(&EventContext, &NativeAccount, &NativeAccount) {
	let subscribed_at = Timestamp::now();
	let handle = async_handler.handle();

	return move |_ctx: &EventContext,
	             outdated_account_data: &NativeAccount,
	             updated_account_data: &NativeAccount| {
		// Only forward events registered after handler initialization
		if subscribed_at.le(&updated_account_data.updated_at) {
			if outdated_account_data.callsign != updated_account_data.callsign {
				let dto = OutboundTelegramMessage {
					// TODO: The chat id must be taken from the corvidx channel properties
					chat_id:     -1001544271932,
					author_name: "system".to_string(),

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
					let _ = tx.send(dto).await;
				});
			}
		}
	};
}

/// @deprecated
/// Prints a notification about callsign and status changes.
fn _on_account_updated(_corvidx: &EventContext, old: &NativeAccount, new: &NativeAccount) {
	if old.is_online && !new.is_online {
		println!("Account {} disconnected.", old.callsign);
	}

	if !old.is_online && new.is_online {
		println!("Account {} connected.", old.callsign);
	}
}

/// If the account is online, prints a notification.
fn on_account_inserted(_corvidx: &EventContext, account: &NativeAccount) {
	if account.is_online {
		println!("Account {} connected.", account.callsign);
	}
}

/// Prints a warning if the reducer failed.
fn on_callsign_set(corvidx: &ReducerEventContext, callsign: &String) {
	if let Status::Failed(err) = &corvidx.event.status {
		eprintln!("Failed to change callsign to {:?}: {}", callsign, err);
	}
}

pub fn subscribe(corvidx: &DbConnection) {
	corvidx.db.native_account().on_insert(on_account_inserted);
	corvidx.reducers.on_set_account_callsign(on_callsign_set);
}
