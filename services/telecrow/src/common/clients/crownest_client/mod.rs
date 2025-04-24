mod config;

use crowlink::clients::crownest;
use spacetimedb_sdk::{DbContext, Error, Identity, credentials};
use std::process;

fn creds_store() -> credentials::File {
	credentials::File::new("Telecrow")
}

/// Saves client user credentials to a file.
fn on_connected(_ctx: &crownest::DbConnection, _identity: Identity, token: &str) {
	if let Err(e) = creds_store().save(token) {
		eprintln!("Failed to save credentials: {:?}", e);
	}
}

/// Prints the error, then exits the process.
fn on_connect_error(_ctx: &crownest::ErrorContext, err: Error) {
	eprintln!("Connection error: {:?}", err);
	process::exit(1);
}

/// Prints a note, then exits the process.
fn on_disconnected(_ctx: &crownest::ErrorContext, err: Option<Error>) {
	if let Some(err) = err {
		eprintln!("Disconnected: {}", err);
		process::exit(1);
	} else {
		println!("Disconnected.");
		process::exit(0);
	}
}

/// Sorts all past messages and print them in timestamp order.
fn on_sub_applied(_crowctx: &crownest::SubscriptionEventContext) {
	println!("Fully connected and all subscriptions applied.");
}

/// Prints the error, then exits the process.
fn on_sub_error(_crowctx: &crownest::ErrorContext, err: Error) {
	eprintln!("Subscription failed: {}", err);
	std::process::exit(1);
}

/// Load credentials from a file and connect to the database.
pub fn connect() -> crownest::DbConnection {
	crownest::DbConnection::builder()
		// Register our `on_connect` callback, which will save our auth token.
		.on_connect(on_connected)
		// Register our `on_connect_error` callback, which will print a message, then exit the process.
		.on_connect_error(on_connect_error)
		// Our `on_disconnect` callback, which will print a message, then exit the process.
		.on_disconnect(on_disconnected)
		// If the user has previously connected, we'll have saved a token in the `on_connect` callback.
		// In that case, we'll load it and pass it to `with_token`,
		// so we can re-authenticate as the same `Identity`.
		.with_token(creds_store().load().expect("Error loading credentials"))
		// Set the database name we chose when we called `spacetime publish`.
		.with_module_name(config::DB_NAME)
		// Set the URI of the SpacetimeDB host that's running our database.
		.with_uri(config::HOST)
		// Finalize configuration and connect!
		.build()
		.expect("Failed to connect")
}

/// Registers subscriptions for all rows of both tables.
pub fn subscribe(crowctx: &crownest::DbConnection) {
	crowctx
		.subscription_builder()
		.on_applied(on_sub_applied)
		.on_error(on_sub_error)
		// Subscribe to SQL queries in order to construct a local partial replica of the database.
		.subscribe(["SELECT * FROM user", "SELECT * FROM message"]);
}
