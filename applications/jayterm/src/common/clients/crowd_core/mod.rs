use std::process;

use crowdcomm::{
	corvidx::{
		self, AccountProfileTableAccess, ForeignAccountTableAccess, LocalAccountTableAccess,
		MessageAuthorId, MessageTableAccess, traits::DisplayName,
	},
	get_env_config,
};
use spacetimedb_sdk::{DbContext, Error, Identity, Table, credentials};

pub fn print_message(ctx: &impl corvidx::RemoteDbContext, message: &corvidx::Message) {
	let sender = match &message.author_id {
		| MessageAuthorId::LocalAccountId(author_id) => ctx
			.db()
			.local_account()
			.id()
			.find(&author_id)
			.map(|account| account.display_name(ctx))
			.unwrap_or_else(|| "unknown".to_string()),

		| MessageAuthorId::ForeignAccountId(author_id) => ctx
			.db()
			.foreign_account()
			.id()
			.find(&author_id)
			.map(|account| account.display_name(ctx))
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
	if let Some(env_config) = get_env_config() {
		corvidx::DbConnection::builder()
		.on_connect(on_connected)
		.on_connect_error(on_connect_error)
		.on_disconnect(on_disconnected)
		// If the account has previously connected, we'll have saved a token in the `on_connect` callback.
		// In that case, we'll load it and pass it to `with_token`,
		// so we can re-authenticate as the same `Identity`.
		.with_token(creds_store().load().expect("Error loading credentials"))
		.with_module_name(env_config.modules.crowspace.name)
		.with_uri(env_config.host)
		.build()
		.expect("Failed to connect")
	} else {
		panic!(
			"❌ Missing environment variables! Check your .env file and .env.example reference."
		);
	}
}

/// Saves client account credentials to a file.
fn on_connected(_ctx: &corvidx::DbConnection, _identity: Identity, token: &str) {
	if let Err(e) = creds_store().save(token) {
		eprintln!("Failed to save credentials: {:?}", e);
	}
}

/// Prints the error, then exits the process.
fn on_connect_error(_ctx: &corvidx::ErrorContext, err: Error) {
	eprintln!("Connection error: {:?}", err);
	process::exit(1);
}

/// Prints a note, then exits the process.
fn on_disconnected(_ctx: &corvidx::ErrorContext, err: Option<Error>) {
	if let Some(err) = err {
		eprintln!("Disconnected: {}", err);
		process::exit(1);
	} else {
		println!("Disconnected.");
		process::exit(0);
	}
}

fn on_sub_applied(ctx: &corvidx::SubscriptionEventContext) {
	let mut messages = ctx.db.message().iter().collect::<Vec<_>>();

	messages.sort_by_key(|m| m.sent_at);

	for message in messages {
		print_message(ctx, &message);
	}

	println!("\nFully connected and all subscriptions applied.");
	println!("Use /callsign to set your callsign, or type a message!\n");

	let foreign_accounts = ctx.db.foreign_account().iter().collect::<Vec<_>>();

	for account in foreign_accounts {
		println!("\n{:?}", account)
	}

	let profiles = ctx.db.account_profile().iter().collect::<Vec<_>>();

	for profile in profiles {
		println!("\n{:?}", profile)
	}
}

/// Prints the error, then exits the process.
fn on_sub_error(_ctx: &corvidx::ErrorContext, err: Error) {
	eprintln!("Subscription failed: {}", err);
	std::process::exit(1);
}

pub fn subscribe_to_tables(ctx: &corvidx::DbConnection) {
	ctx.subscription_builder()
		.on_applied(on_sub_applied)
		.on_error(on_sub_error)
		.subscribe([
			"SELECT * FROM local_account",
			"SELECT * FROM foreign_account",
			"SELECT * FROM message",
			"SELECT * FROM account_profile",
		]);
}
