use std::process;

use crowcomm::{
	crowd_core::{
		self, AccountProfileTableAccess, ForeignAccountTableAccess, LocalAccountTableAccess,
		MessageAuthorId, MessageTableAccess, send_message, set_callsign, traits::DisplayName,
	},
	get_env_config,
};
use spacetimedb_sdk::{
	DbContext, Error, Event, Identity, Status, Table, TableWithPrimaryKey, credentials,
};

// !	CONNECTION SETUP

fn creds_store() -> credentials::File {
	credentials::File::new("console.crowd-credentials")
}

/// Saves client account credentials to a file.
fn on_connected(_ctx: &crowd_core::DbConnection, _identity: Identity, token: &str) {
	if let Err(e) = creds_store().save(token) {
		eprintln!("Failed to save credentials: {:?}", e);
	}
}

/// Prints the error, then exits the process.
fn on_connect_error(_ctx: &crowd_core::ErrorContext, err: Error) {
	eprintln!("Connection error: {:?}", err);
	process::exit(1);
}

/// Prints a note, then exits the process.
fn on_disconnected(_ctx: &crowd_core::ErrorContext, err: Option<Error>) {
	if let Some(err) = err {
		eprintln!("Disconnected: {}", err);
		process::exit(1);
	} else {
		println!("Disconnected.");
		process::exit(0);
	}
}

// TODO: Extract to Account entity

/// If the account is online, prints a notification.
fn on_account_inserted(_ctx: &crowd_core::EventContext, account: &crowd_core::LocalAccount) {
	if account.is_online {
		println!("Account {} connected.", account.callsign);
	}
}

/// Prints a notification about callsign and status changes.
fn on_account_updated(
	_ctx: &crowd_core::EventContext, old: &crowd_core::LocalAccount, new: &crowd_core::LocalAccount,
) {
	if old.callsign != new.callsign {
		println!(
			"Account {} changed callsign from {} to {}.",
			old.id, old.callsign, new.callsign,
		);
	}

	if old.is_online && !new.is_online {
		println!("Account {} disconnected.", old.callsign);
	}

	if !old.is_online && new.is_online {
		println!("Account {} connected.", old.callsign);
	}
}

/// Prints a warning if the reducer failed.
fn on_callsign_set(ctx: &crowd_core::ReducerEventContext, callsign: &String) {
	if let Status::Failed(err) = &ctx.event.status {
		eprintln!("Failed to change callsign to {:?}: {}", callsign, err);
	}
}

//*	MESSAGE SUBSCRIPTIONS

fn print_message(ctx: &impl crowd_core::RemoteDbContext, message: &crowd_core::Message) {
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

/// Prints new messages.
fn on_message_inserted(ctx: &crowd_core::EventContext, message: &crowd_core::Message) {
	if let Event::Reducer(_) = ctx.event {
		print_message(ctx, message)
	}
}

/// Prints a warning if the reducer failed.
fn on_message_sent(ctx: &crowd_core::ReducerEventContext, text: &String) {
	if let Status::Failed(err) = &ctx.event.status {
		eprintln!("Failed to send message {:?}: {}", text, err);
	}
}

// !	TABLE SUBSCRIPTIONS

/// Sorts all past messages and print them in timestamp order.
fn on_sub_applied(ctx: &crowd_core::SubscriptionEventContext) {
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
fn on_sub_error(_ctx: &crowd_core::ErrorContext, err: Error) {
	eprintln!("Subscription failed: {}", err);
	std::process::exit(1);
}

fn subscribe_to_tables(ctx: &crowd_core::DbConnection) {
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

// !	USER INPUT

/// Reads each line of standard input, and either executes a command or sends a
/// message as appropriate.
fn account_input_loop(ctx: &crowd_core::DbConnection) {
	for line in std::io::stdin().lines() {
		let Ok(line) = line else {
			panic!("Failed to read from stdin.");
		};

		if let Some(callsign) = line.strip_prefix("/callsign ") {
			ctx.reducers.set_callsign(callsign.to_string()).unwrap();
		} else {
			ctx.reducers.send_message(line).unwrap();
		}
	}
}

// !	GENERAL

/// Registers all the callbacks the app will use to respond to database events.
fn register_callbacks(ctx: &crowd_core::DbConnection) {
	// When a new account joins, print a notification.
	ctx.db.local_account().on_insert(on_account_inserted);

	// When a account's status changes, print a notification.
	ctx.db.local_account().on_update(on_account_updated);

	// When a new message is received, print it.
	ctx.db.message().on_insert(on_message_inserted);

	// When we fail to set our callsign, print a warning.
	ctx.reducers.on_set_callsign(on_callsign_set);

	// When we fail to send a message, print a warning.
	ctx.reducers.on_send_message(on_message_sent);
}

/// Load credentials from a file and connect to the database.
fn connect_to_db() -> crowd_core::DbConnection {
	if let Some(env_config) = get_env_config() {
		crowd_core::DbConnection::builder()
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

fn main() {
	let _ = dotenvy::dotenv();

	// Connect to the database
	let ctx = connect_to_db();

	register_callbacks(&ctx);
	subscribe_to_tables(&ctx);
	ctx.run_threaded();
	account_input_loop(&ctx);
}
