use std::process;

use crowcomm::{
	crowspace::{
		self, AccountTableAccess, MessageTableAccess, PublicProfileTableAccess, send_message,
		set_callsign, traits::DisplayName,
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
fn on_connected(_ctx: &crowspace::DbConnection, _identity: Identity, token: &str) {
	if let Err(e) = creds_store().save(token) {
		eprintln!("Failed to save credentials: {:?}", e);
	}
}

/// Prints the error, then exits the process.
fn on_connect_error(_ctx: &crowspace::ErrorContext, err: Error) {
	eprintln!("Connection error: {:?}", err);
	process::exit(1);
}

/// Prints a note, then exits the process.
fn on_disconnected(_ctx: &crowspace::ErrorContext, err: Option<Error>) {
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
fn on_account_inserted(_ctx: &crowspace::EventContext, account: &crowspace::Account) {
	if account.is_online {
		println!("Account {} connected.", account.callsign);
	}
}

/// Prints a notification about callsign and status changes.
fn on_account_updated(
	_ctx: &crowspace::EventContext, old: &crowspace::Account, new: &crowspace::Account,
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
fn on_callsign_set(ctx: &crowspace::ReducerEventContext, callsign: &String) {
	if let Status::Failed(err) = &ctx.event.status {
		eprintln!("Failed to change callsign to {:?}: {}", callsign, err);
	}
}

//*	MESSAGE SUBSCRIPTIONS

fn print_message(ctx: &impl crowspace::RemoteDbContext, message: &crowspace::Message) {
	let sender = ctx
		.db()
		.account()
		.id()
		.find(&message.sender.clone())
		.map(|account| account.display_name(ctx))
		.unwrap_or_else(|| "unknown".to_string());

	println!("{}: {}", sender, message.text);
}

/// Prints new messages.
fn on_message_inserted(ctx: &crowspace::EventContext, message: &crowspace::Message) {
	if let Event::Reducer(_) = ctx.event {
		print_message(ctx, message)
	}
}

/// Prints a warning if the reducer failed.
fn on_message_sent(ctx: &crowspace::ReducerEventContext, text: &String) {
	if let Status::Failed(err) = &ctx.event.status {
		eprintln!("Failed to send message {:?}: {}", text, err);
	}
}

// !	TABLE SUBSCRIPTIONS

/// Sorts all past messages and print them in timestamp order.
fn on_sub_applied(ctx: &crowspace::SubscriptionEventContext) {
	let mut messages = ctx.db.message().iter().collect::<Vec<_>>();

	messages.sort_by_key(|m| m.sent_at);

	for message in messages {
		print_message(ctx, &message);
	}

	println!("Fully connected and all subscriptions applied.");
	println!("Use /callsign to set your callsign, or type a message!");
}

/// Prints the error, then exits the process.
fn on_sub_error(_ctx: &crowspace::ErrorContext, err: Error) {
	eprintln!("Subscription failed: {}", err);
	std::process::exit(1);
}

fn subscribe_to_tables(ctx: &crowspace::DbConnection) {
	ctx.subscription_builder()
		.on_applied(on_sub_applied)
		.on_error(on_sub_error)
		.subscribe([
			"SELECT * FROM account",
			"SELECT * FROM external_account",
			"SELECT * FROM message",
			"SELECT * FROM public_profile",
		]);
}

// !	USER INPUT

/// Reads each line of standard input, and either executes a command or sends a
/// message as appropriate.
fn account_input_loop(ctx: &crowspace::DbConnection) {
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
fn register_callbacks(ctx: &crowspace::DbConnection) {
	// When a new account joins, print a notification.
	ctx.db.account().on_insert(on_account_inserted);

	// When a account's status changes, print a notification.
	ctx.db.account().on_update(on_account_updated);

	// When a new message is received, print it.
	ctx.db.message().on_insert(on_message_inserted);

	// When we fail to set our callsign, print a warning.
	ctx.reducers.on_set_callsign(on_callsign_set);

	// When we fail to send a message, print a warning.
	ctx.reducers.on_send_message(on_message_sent);
}

/// Load credentials from a file and connect to the database.
fn connect_to_db() -> crowspace::DbConnection {
	if let Some(env_config) = get_env_config() {
		crowspace::DbConnection::builder()
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
