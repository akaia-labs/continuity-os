use crowdcomm_sdk::corvidx::stdb::{DbConnection, ReducerEventContext, send_message};
use spacetimedb_sdk::Status;

/// Prints a warning if the reducer failed.
fn on_message_sent(corvidx: &ReducerEventContext, text: &String) {
	if let Status::Failed(err) = &corvidx.event.status {
		eprintln!("Failed to send message {:?}: {}", text, err);
	}
}

pub fn subscribe(corvidx: &DbConnection) {
	corvidx.reducers.on_send_message(on_message_sent);
}
