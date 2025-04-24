mod common;
pub mod entities;

use crowlink::clients::crownest::{self, *};
use log::log;
use spacetimedb_sdk::{DbContext, Error, Table, TableWithPrimaryKey};

use common::clients::{
	crownest_client,
	telegram_bot_client::{self, *},
};

use entities::{message_subscriptions, user_subscriptions};

pub type TelecrowError = Box<dyn std::error::Error + Send + Sync>;

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

fn telegram_bot_pipeline(ctx: &crownest::DbConnection) {
	// for line in std::io::stdin().lines() {
	// 	let Ok(line) = line else {
	// 		panic!("Failed to read from stdin.");
	// 	};

	// 	if let Some(name) = line.strip_prefix("/name ") {
	// 		ctx.reducers.set_name(name.to_string()).unwrap();
	// 	} else {
	// 		ctx.reducers.send_message(line).unwrap();
	// 	}
	// }
}

async fn process_text_message(
	bot: telegram_bot_client::Bot, tg_user: telegram_bot_client::User, message_text: String,
) -> Result<(), Error> {
	log::info!(
		"@{:#?}: {}",
		tg_user.username.clone().unwrap_or(tg_user.id.to_string()),
		message_text
	);

	/*
	   The id of a chat with a user is the same as his telegram_id
	   from the bot's perspective.

	   Injected dependencies:
	   - Bot is provided by the Dispatcher::dispatch
	   - User is provided by the (1)
	   - String is provided by the (2)
	*/
	let _ = bot.send_message(
		tg_user.id,
		format!(
			"@{:#?}: {}",
			tg_user.username.unwrap_or(tg_user.id.to_string()),
			message_text
		),
	);

	Ok(())
}

#[tokio::main]
async fn main() -> Result<(), TelecrowError> {
	dotenvy::dotenv()?;
	pretty_env_logger::init();

	log::info!("Initializing DB connection...");
	let crownest_context = crownest_client::connect();

	// Register callbacks to run in response to database events.
	register_callbacks(&crownest_context);

	// Subscribe to SQL queries in order to construct a local partial replica of the database.
	subscribe_to_tables(&crownest_context);

	// Spawn a thread, where the connection will process messages and invoke callbacks.
	crownest_context.run_threaded();

	log::info!("Initializing Telegram bot...");
	let tlx_bot = telegram_bot_client::Bot::from_env();

	let tlx_schema = telegram_bot_client::Update::filter_message()
	/*
	   Inject the `User` object representing the author of an incoming
	   message into every successive handler function (1)
	*/
	.filter_map(|update: telegram_bot_client::Update| update.from().cloned())
	.branch(
		/*
		   Use filter_text method of MessageFilterExt to accept
		   only textual messages. Others will be ignored by this handler (2)
		*/
		telegram_bot_client::Message::filter_text().endpoint(process_text_message),
	);

	// telegram_bot_pipeline(&crownest_context);

	log::info!("Starting Telegram bot...");
	telegram_bot_client::Dispatcher::builder(tlx_bot, tlx_schema)
		.build()
		.dispatch()
		.await;

	Ok(())
}
