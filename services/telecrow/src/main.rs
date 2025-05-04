pub mod common;
pub mod entities;
pub mod features;

use std::sync::Arc;

use crowcomm::telegram;
use dotenvy::dotenv;
use teloxide::{
	Bot,
	dispatching::{HandlerExt, UpdateFilterExt},
	dptree,
	prelude::Dispatcher,
};

use crate::{
	common::{clients::crowspace_client, runtime, runtime::TelecrowError},
	entities::{crowspace_account, crowspace_message},
	features::telegram_relay,
};

#[tokio::main]
async fn main() -> Result<(), TelecrowError> {
	dotenv()?;
	pretty_env_logger::init();
	println!("\n⏳ Initializing clients...\n");

	let async_handler = runtime::new_async_handler();
	let crowspace_connection = Arc::new(crowspace_client::connect());
	let telegram_relay_bot = Bot::from_env();

	println!("⏳ Initializing subscriptions...\n");
	crowspace_client::subscribe(&crowspace_connection);
	crowspace_account::subscribe(&crowspace_connection);
	crowspace_message::subscribe(&crowspace_connection);
	crowspace_connection.run_threaded();

	telegram_relay::subscribe(
		&crowspace_connection,
		async_handler.clone(),
		telegram_relay_bot.clone(),
	);

	let telegram_relay_handler = dptree::entry()
		.branch(dptree::entry().map(|update: telegram::Update| update.from().cloned()))
		.branch(
			telegram::Update::filter_message()
			.branch(
				dptree::entry()
				.filter_command::<telegram_relay::BasicCommand>()
				.endpoint(telegram_relay::on_basic_command),
			)
			// Injecting the `User` object representing the author of an incoming message
			.filter_map(|update: telegram::Update| update.from().cloned())
			.branch(
				dptree::endpoint(
					telegram_relay::handle_messages(crowspace_connection.clone())
				),
			),
		);

	println!("⌛ Starting Telegram bot dispatcher...\n");

	Dispatcher::builder(telegram_relay_bot, telegram_relay_handler)
		.build()
		.dispatch()
		.await;

	Ok(())
}
