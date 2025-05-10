use std::process;

use crowdcomm_sdk::corvidx::{DbConnection, ErrorContext, SubscriptionEventContext};
use spacetimedb_sdk::{Error, Identity};

use super::authentication;

/// Saves client account credentials to a file.
pub fn on_connected(_corvidx: &DbConnection, _identity: Identity, token: &str) {
	if let Err(e) = authentication::credential_store().save(token) {
		eprintln!("❌ Failed to save credentials: {:?}", e);
	}
}

/// Prints the error, then exits the process.
pub fn on_connect_error(_corvidx: &ErrorContext, err: Error) {
	eprintln!("❌ Connection error: {:?}", err);
	process::exit(1);
}

/// Prints a note, then exits the process.
pub fn on_disconnected(_corvidx: &ErrorContext, err: Option<Error>) {
	if let Some(err) = err {
		eprintln!("❌ Disconnected: {}", err);
		process::exit(1);
	} else {
		println!("Disconnected.");
		process::exit(0);
	}
}

pub fn on_sub_applied(_crowspace_ctx: &SubscriptionEventContext) {
	println!("✅ Fully connected and all subscriptions applied.\n");
	println!("🚀 ONLINE!\n");
}

/// Prints the error, then exits the process.
pub fn on_sub_error(_crowspace_ctx: &ErrorContext, err: Error) {
	eprintln!("❌ Subscription failed: {}", err);
	std::process::exit(1);
}
