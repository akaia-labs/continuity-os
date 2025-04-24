mod common;
pub mod entities;

use common::clients::crownest_client;
use crowlink::clients::crownest::{self, *};

use entities::{message_subscriptions, user_subscriptions};
use spacetimedb_sdk::{DbContext, Error, Table, TableWithPrimaryKey};

/*
!	TABLE SUBSCRIPTIONS
*/

/// Sorts all past messages and print them in timestamp order.
fn on_sub_applied(ctx: &crownest::SubscriptionEventContext) {
	let mut messages = ctx.db.message().iter().collect::<Vec<_>>();

	messages.sort_by_key(|m| m.sent);

	for message in messages {
		message_subscriptions::print_message(ctx, &message);
	}

	println!("Fully connected and all subscriptions applied.");
	println!("Use /name to set your name, or type a message!");
}

/// Prints the error, then exits the process.
fn on_sub_error(_ctx: &crownest::ErrorContext, err: Error) {
	eprintln!("Subscription failed: {}", err);
	std::process::exit(1);
}

/// Registers subscriptions for all rows of both tables.
fn subscribe_to_tables(ctx: &crownest::DbConnection) {
	ctx.subscription_builder()
		.on_applied(on_sub_applied)
		.on_error(on_sub_error)
		.subscribe(["SELECT * FROM user", "SELECT * FROM message"]);
}

/*
!	USER INPUT
*/

/// Reads each line of standard input, and either executes a command or sends a message as appropriate.
fn user_input_loop(ctx: &crownest::DbConnection) {
	for line in std::io::stdin().lines() {
		let Ok(line) = line else {
			panic!("Failed to read from stdin.");
		};

		if let Some(name) = line.strip_prefix("/name ") {
			ctx.reducers.set_name(name.to_string()).unwrap();
		} else {
			ctx.reducers.send_message(line).unwrap();
		}
	}
}

/*
!	GENERAL
*/

/// Registers all the callbacks the app will use to respond to database events.
fn register_callbacks(ctx: &crownest::DbConnection) {
	// When a new user joins, print a notification.
	ctx.db
		.user()
		.on_insert(user_subscriptions::on_user_inserted);

	// When a user's status changes, print a notification.
	ctx.db.user().on_update(user_subscriptions::on_user_updated);

	// When a new message is received, print it.
	ctx.db
		.message()
		.on_insert(message_subscriptions::on_message_inserted);

	// When we fail to set our name, print a warning.
	ctx.reducers.on_set_name(user_subscriptions::on_name_set);

	// When we fail to send a message, print a warning.
	ctx.reducers
		.on_send_message(message_subscriptions::on_message_sent);
}

fn main() {
	// Connect to the database
	let crownest_context = crownest_client::connect();

	// Register callbacks to run in response to database events.
	register_callbacks(&crownest_context);

	// Subscribe to SQL queries in order to construct a local partial replica of the database.
	subscribe_to_tables(&crownest_context);

	// Spawn a thread, where the connection will process messages and invoke callbacks.
	crownest_context.run_threaded();

	// Handle CLI input
	user_input_loop(&crownest_context);
}
