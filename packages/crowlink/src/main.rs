use std::process;

use crowlink::clients::crownest::{
	self, MessageTableAccess, UserTableAccess, send_message, set_name,
};

use spacetimedb_sdk::{
	DbContext, Error, Event, Identity, Status, Table, TableWithPrimaryKey, credentials,
};

/*
!	CONNECTION CONFIG
*/

/// The URI of the SpacetimeDB instance hosting the server module.
const HOST: &str = "http://localhost:3333";

/// The database name under which the module is published.
const DB_NAME: &str = "crowd";

/*
!	CONNECTION SETUP
*/

fn creds_store() -> credentials::File {
	credentials::File::new(DB_NAME)
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

/*
!	USER SUBSCRIPTIONS
*/

/// Returns the user's name, or their identity if they have no name.
fn user_name_or_identity(user: &crownest::User) -> String {
	user.name
		.clone()
		.unwrap_or_else(|| user.identity.to_hex().to_string())
}

/// If the user is online, prints a notification.
fn on_user_inserted(_ctx: &crownest::EventContext, user: &crownest::User) {
	if user.online {
		println!("User {} connected.", user_name_or_identity(user));
	}
}

/// Prints a notification about name and status changes.
fn on_user_updated(_ctx: &crownest::EventContext, old: &crownest::User, new: &crownest::User) {
	if old.name != new.name {
		println!(
			"User {} renamed to {}.",
			user_name_or_identity(old),
			user_name_or_identity(new)
		);
	}

	if old.online && !new.online {
		println!("User {} disconnected.", user_name_or_identity(new));
	}

	if !old.online && new.online {
		println!("User {} connected.", user_name_or_identity(new));
	}
}

/// Prints a warning if the reducer failed.
fn on_name_set(ctx: &crownest::ReducerEventContext, name: &String) {
	if let Status::Failed(err) = &ctx.event.status {
		eprintln!("Failed to change name to {:?}: {}", name, err);
	}
}

/*
!	MESSAGE SUBSCRIPTIONS
*/

fn print_message(ctx: &impl crownest::RemoteDbContext, message: &crownest::Message) {
	let sender = ctx
		.db()
		.user()
		.identity()
		.find(&message.sender.clone())
		.map(|u| user_name_or_identity(&u))
		.unwrap_or_else(|| "unknown".to_string());

	println!("{}: {}", sender, message.text);
}

/// Prints new messages.
fn on_message_inserted(ctx: &crownest::EventContext, message: &crownest::Message) {
	if let Event::Reducer(_) = ctx.event {
		print_message(ctx, message)
	}
}

/// Prints a warning if the reducer failed.
fn on_message_sent(ctx: &crownest::ReducerEventContext, text: &String) {
	if let Status::Failed(err) = &ctx.event.status {
		eprintln!("Failed to send message {:?}: {}", text, err);
	}
}

/*
!	TABLE SUBSCRIPTIONS
*/

/// Sorts all past messages and print them in timestamp order.
fn on_sub_applied(ctx: &crownest::SubscriptionEventContext) {
	let mut messages = ctx.db.message().iter().collect::<Vec<_>>();

	messages.sort_by_key(|m| m.sent);

	for message in messages {
		print_message(ctx, &message);
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
	ctx.db.user().on_insert(on_user_inserted);

	// When a user's status changes, print a notification.
	ctx.db.user().on_update(on_user_updated);

	// When a new message is received, print it.
	ctx.db.message().on_insert(on_message_inserted);

	// When we fail to set our name, print a warning.
	ctx.reducers.on_set_name(on_name_set);

	// When we fail to send a message, print a warning.
	ctx.reducers.on_send_message(on_message_sent);
}

/// Load credentials from a file and connect to the database.
fn connect_to_db() -> crownest::DbConnection {
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
        .with_module_name(DB_NAME)
        // Set the URI of the SpacetimeDB host that's running our database.
        .with_uri(HOST)
        // Finalize configuration and connect!
        .build()
        .expect("Failed to connect")
}

fn main() {
	// Connect to the database
	let ctx = connect_to_db();

	// Register callbacks to run in response to database events.
	register_callbacks(&ctx);

	// Subscribe to SQL queries in order to construct a local partial replica of the database.
	subscribe_to_tables(&ctx);

	// Spawn a thread, where the connection will process messages and invoke callbacks.
	ctx.run_threaded();

	// Handle CLI input
	user_input_loop(&ctx);
}
