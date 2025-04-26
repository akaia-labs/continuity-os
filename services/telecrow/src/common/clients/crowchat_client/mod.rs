use core::panic;
use crowtocol_rs::{crowchat, get_env_config};
use spacetimedb_sdk::{DbContext, Error, Identity, credentials};
use std::process;

fn creds_store() -> credentials::File {
	credentials::File::new("telecrow.credentials")
}

/// Saves client user credentials to a file.
fn on_connected(_ctx: &crowchat::DbConnection, _identity: Identity, token: &str) {
	if let Err(e) = creds_store().save(token) {
		eprintln!("❌ Failed to save credentials: {:?}", e);
	}
}

/// Prints the error, then exits the process.
fn on_connect_error(_ctx: &crowchat::ErrorContext, err: Error) {
	eprintln!("❌ Connection error: {:?}", err);
	process::exit(1);
}

/// Prints a note, then exits the process.
fn on_disconnected(_ctx: &crowchat::ErrorContext, err: Option<Error>) {
	if let Some(err) = err {
		eprintln!("❌ Disconnected: {}", err);
		process::exit(1);
	} else {
		println!("Disconnected.");
		process::exit(0);
	}
}

fn on_sub_applied(_crowctx: &crowchat::SubscriptionEventContext) {
	println!("✅ Fully connected and all subscriptions applied.\n");
	println!("🚀 ONLINE!\n");
}

/// Prints the error, then exits the process.
fn on_sub_error(_crowctx: &crowchat::ErrorContext, err: Error) {
	eprintln!("❌ Subscription failed: {}", err);
	std::process::exit(1);
}

/// Load credentials from a file and connect to the database.
pub fn connect() -> crowchat::DbConnection {
	if let Some(env_config) = get_env_config() {
		crowchat::DbConnection::builder()
		.on_connect(on_connected)
		.on_connect_error(on_connect_error)
		.on_disconnect(on_disconnected)
		// If the user has previously connected, we'll have saved a token in the `on_connect` callback.
		// In that case, we'll load it and pass it to `with_token`,
		// so we can re-authenticate as the same `Identity`.
		.with_token(creds_store().load().expect("Error loading credentials"))
		.with_module_name(env_config.modules.chat.name)
		.with_uri(env_config.host)
		.build()
		.expect("Failed to connect")
	} else {
		panic!(
			"❌ Missing environment variables! Check your .env file and .env.example reference."
		);
	}
}

/// Registers subscriptions for all rows of both tables.
pub fn subscribe(crowctx: &crowchat::DbConnection) {
	crowctx
		.subscription_builder()
		.on_applied(on_sub_applied)
		.on_error(on_sub_error)
		// Subscribe to SQL queries in order to construct a local partial replica of the database.
		.subscribe(["SELECT * FROM user", "SELECT * FROM message"]);
}
