mod common;
pub mod entities;

use crowlink::clients::crownest::{self, *};
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
fn on_sub_applied(crowctx: &crownest::SubscriptionEventContext) {
	let mut messages = crowctx.db.message().iter().collect::<Vec<_>>();

	messages.sort_by_key(|m| m.sent);

	for message in messages {
		message_subscriptions::print_message(crowctx, &message);
	}

	println!("Fully connected and all subscriptions applied.");
	println!("Use /name to set your name, or type a message!");
}

/// Prints the error, then exits the process.
fn on_sub_error(_crowctx: &crownest::ErrorContext, err: Error) {
	eprintln!("Subscription failed: {}", err);
	std::process::exit(1);
}

/// Registers subscriptions for all rows of both tables.
fn subscribe_to_tables(crowctx: &crownest::DbConnection) {
	crowctx
		.subscription_builder()
		.on_applied(on_sub_applied)
		.on_error(on_sub_error)
		// Subscribe to SQL queries in order to construct a local partial replica of the database.
		.subscribe(["SELECT * FROM user", "SELECT * FROM message"]);
}

/*
!	GENERAL
*/

/// Registers all the callbacks the app will use to respond to database events.
fn register_callbacks(crowctx: &crownest::DbConnection, tg_bot: &telegram_bot_client::Bot) {
	crowctx
		.db
		.user()
		.on_insert(user_subscriptions::on_user_inserted);

	crowctx
		.db
		.user()
		.on_update(user_subscriptions::on_user_updated);

	crowctx
		.db
		.message()
		.on_insert(message_subscriptions::handle_telegram_forward(
			tg_bot.clone(),
		));

	crowctx
		.reducers
		.on_set_name(user_subscriptions::on_name_set);

	crowctx
		.reducers
		.on_send_message(message_subscriptions::on_message_sent);
}

fn on_tg_text_message(crowctx: &crownest::DbConnection, tg_message: telegram_bot_client::Message) {
	if let Some(text) = tg_message.text() {
		crowctx.reducers.send_message(text.to_owned()).unwrap();
	}
}

async fn process_text_message(
	_tg_bot: telegram_bot_client::Bot, tg_user: telegram_bot_client::User, message_text: String,
) -> Result<(), TelecrowError> {
	log::info!(
		"@{:#?}: {}",
		tg_user.username.clone().unwrap_or(tg_user.id.to_string()),
		message_text
	);

	// let _message = tg_bot
	// 	.send_message(
	// 		tg_user.id,
	// 		format!(
	// 			"@{:#?}: {}",
	// 			tg_user.username.unwrap_or(tg_user.id.to_string()),
	// 			message_text
	// 		),
	// 	)
	// 	.await
	// 	.unwrap();

	Ok(())
}

#[tokio::main]
async fn main() -> Result<(), TelecrowError> {
	dotenvy::dotenv()?;
	pretty_env_logger::init();

	log::info!("Initializing DB connection...");
	let crowctx = crownest_client::connect();

	log::info!("Initializing Telegram bot...");
	let telegram_bot = telegram_bot_client::Bot::from_env();

	register_callbacks(&crowctx, &telegram_bot);
	subscribe_to_tables(&crowctx);
	crowctx.run_threaded();

	let teloxide_schema = telegram_bot_client::Update::filter_message()
	.inspect(move |msg: telegram_bot_client::Message| on_tg_text_message(&crowctx, msg))
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

	log::info!("Starting Telegram bot...");
	telegram_bot_client::Dispatcher::builder(telegram_bot, teloxide_schema)
		.build()
		.dispatch()
		.await;

	Ok(())
}
