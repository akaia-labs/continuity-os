use std::process;

use crowdcomm_sdk::{
	corvid_subsystem_config::{self, CorvidSubsystemConfig},
	corvidx::{
		self, AccountProfileTableAccess, ForeignAccountTableAccess, LocalAccountTableAccess,
		MessageAuthorId, MessageTableAccess, traits::DisplayName,
	},
};
use spacetimedb_sdk::{DbContext, Error, Identity, Table, credentials};

pub fn print_message(corvidx: &impl corvidx::RemoteDbContext, message: &corvidx::Message) {
	let sender = match &message.author_id {
		| MessageAuthorId::LocalAccountId(author_id) => corvidx
			.db()
			.local_account()
			.id()
			.find(&author_id)
			.map(|account| account.display_name(corvidx))
			.unwrap_or_else(|| "unknown".to_string()),

		| MessageAuthorId::ForeignAccountId(author_id) => corvidx
			.db()
			.foreign_account()
			.id()
			.find(&author_id)
			.map(|account| account.display_name(corvidx))
			.unwrap_or_else(|| "unknown".to_string()),

		| MessageAuthorId::System => "system".to_string(),
		| MessageAuthorId::Unknown => "unknown".to_string(),
	};

	println!("{}: {}", sender, message.text);
}

fn creds_store() -> credentials::File {
	credentials::File::new("console.crowd-credentials")
}

/// Load credentials from a file and connect to the database.
pub fn connect_to_db() -> corvidx::DbConnection {
	let CorvidSubsystemConfig {
		module_host,
		components,
		..
	} = corvid_subsystem_config::get();

	corvidx::DbConnection::builder()
		.on_connect(on_connected)
		.on_connect_error(on_connect_error)
		.on_disconnect(on_disconnected)
		// If the account has previously connected, we'll have saved a token in the `on_connect` callback.
		// In that case, we'll load it and pass it to `with_token`,
		// so we can re-authenticate as the same `Identity`.
		.with_token(creds_store().load().expect("Error loading credentials"))
		.with_module_name(components.corvidx.db_name)
		.with_uri(module_host)
		.build()
		.expect("Failed to connect")
}

/// Saves client account credentials to a file.
fn on_connected(_corvidx: &corvidx::DbConnection, _identity: Identity, token: &str) {
	if let Err(e) = creds_store().save(token) {
		eprintln!("Failed to save credentials: {:?}", e);
	}
}

/// Prints the error, then exits the process.
fn on_connect_error(_corvidx: &corvidx::ErrorContext, err: Error) {
	eprintln!("Connection error: {:?}", err);
	process::exit(1);
}

/// Prints a note, then exits the process.
fn on_disconnected(_corvidx: &corvidx::ErrorContext, err: Option<Error>) {
	if let Some(err) = err {
		eprintln!("Disconnected: {}", err);
		process::exit(1);
	} else {
		println!("Disconnected.");
		process::exit(0);
	}
}

fn on_sub_applied(corvidx: &corvidx::SubscriptionEventContext) {
	let mut messages = corvidx.db.message().iter().collect::<Vec<_>>();

	messages.sort_by_key(|m| m.sent_at);

	for message in messages {
		print_message(corvidx, &message);
	}

	println!("\nFully connected and all subscriptions applied.");
	println!("Use /callsign to set your callsign, or type a message!\n");

	let foreign_accounts = corvidx.db.foreign_account().iter().collect::<Vec<_>>();

	for account in foreign_accounts {
		println!("\n{:?}", account)
	}

	let profiles = corvidx.db.account_profile().iter().collect::<Vec<_>>();

	for profile in profiles {
		println!("\n{:?}", profile)
	}
}

/// Prints the error, then exits the process.
fn on_sub_error(_corvidx: &corvidx::ErrorContext, err: Error) {
	eprintln!("Subscription failed: {}", err);
	std::process::exit(1);
}

pub fn subscribe_to_tables(corvidx: &corvidx::DbConnection) {
	corvidx
		.subscription_builder()
		.on_applied(on_sub_applied)
		.on_error(on_sub_error)
		.subscribe([
			"SELECT * FROM local_account",
			"SELECT * FROM foreign_account",
			"SELECT * FROM account_profile",
			// "SELECT * FROM account_linking_request",
			"SELECT * FROM message",
			// "SELECT * FROM message_channel",
		]);
}
