use std::process;

use crowdcomm_sdk::{
	configuration::corvid_subsystem_config::{self, CorvidSubsystemConfig},
	corvidx::{
		ports::{ProfileResolution, RecordResolver},
		presentation::{DisplayName, Displayable},
		stdb::{
			ActorId, ActorProfileTableAccess, DbConnection, ErrorContext, ExternalActorTableAccess,
			Message, MessageTableAccess, RemoteDbContext, SubscriptionEventContext,
		},
	},
};
use spacetimedb_sdk::{DbContext, Error, Identity, Table, credentials};

pub fn print_message(ctx: &impl RemoteDbContext, message: &Message) {
	let sender = match &message.author {
		| ActorId::Internal(author_id) => author_id
			.resolve(ctx)
			.map(|account| account.display_name(ctx))
			.unwrap_or_else(|| "unknown".to_string()),

		| ActorId::External(author_id) => author_id
			.resolve(ctx)
			.map(|account| {
				account
					.profile(ctx)
					.map(|p| p.display_name())
					.unwrap_or_else(|| "unknown".to_string())
			})
			.unwrap_or_else(|| "unknown".to_string()),
	};

	println!("{}: {}", sender, message.text);
}

fn creds_store() -> credentials::File {
	credentials::File::new("console.crowd-credentials")
}

/// Load credentials from a file and connect to the database.
pub fn connect_to_db() -> DbConnection {
	let CorvidSubsystemConfig {
		module_host,
		components,
		..
	} = corvid_subsystem_config::get();

	DbConnection::builder()
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
fn on_connected(_ctx: &DbConnection, _identity: Identity, token: &str) {
	if let Err(e) = creds_store().save(token) {
		eprintln!("Failed to save credentials: {:?}", e);
	}
}

/// Prints the error, then exits the process.
fn on_connect_error(_ctx: &ErrorContext, err: Error) {
	eprintln!("Connection error: {:?}", err);
	process::exit(1);
}

/// Prints a note, then exits the process.
fn on_disconnected(_ctx: &ErrorContext, err: Option<Error>) {
	if let Some(err) = err {
		eprintln!("Disconnected: {}", err);
		process::exit(1);
	} else {
		println!("Disconnected.");
		process::exit(0);
	}
}

fn on_sub_applied(ctx: &SubscriptionEventContext) {
	let mut messages = ctx.db.message().iter().collect::<Vec<_>>();

	messages.sort_by_key(|m| m.sent_at);

	for message in messages {
		print_message(ctx, &message);
	}

	println!("\nFully connected and all subscriptions applied.");
	println!("Use /callsign to set your callsign, or type a message!\n");

	let external_actors = ctx.db.external_actor().iter().collect::<Vec<_>>();

	for account in external_actors {
		println!("\n{:?}", account)
	}

	let profiles = ctx.db.actor_profile().iter().collect::<Vec<_>>();

	for profile in profiles {
		println!("\n{:?}", profile)
	}
}

/// Prints the error, then exits the process.
fn on_sub_error(_ctx: &ErrorContext, err: Error) {
	eprintln!("Subscription failed: {}", err);
	std::process::exit(1);
}

pub fn subscribe_to_tables(ctx: &DbConnection) {
	ctx.subscription_builder()
		.on_applied(on_sub_applied)
		.on_error(on_sub_error)
		.subscribe([
			"SELECT * FROM external_authentication_request",
			"SELECT * FROM actor_profile",
			"SELECT * FROM external_actor",
			"SELECT * FROM message",
			// "SELECT * FROM message_channel",
			"SELECT * FROM account",
		]);
}
