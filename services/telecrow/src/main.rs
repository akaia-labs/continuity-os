pub mod common;
pub mod entities;
pub mod features;

use std::sync::Arc;

use crowcomm::telegram;
use dotenvy::dotenv;
use entities::telegram_user;
use teloxide::{
	Bot,
	dispatching::{HandlerExt, UpdateFilterExt},
	dptree,
	prelude::Dispatcher,
};

use crate::{
	common::{clients::crowd_core_client, runtime, runtime::TelecrowError},
	entities::{local_account, local_message},
	features::telegram_relay,
};

#[tokio::main]
async fn main() -> Result<(), TelecrowError> {
	dotenv()?;
	pretty_env_logger::init();
	println!("\n⏳ Initializing clients...\n");

	let async_handler = runtime::new_async_handler();
	let core_connection = Arc::new(crowd_core_client::connect());
	let telegram_relay_bot = Bot::from_env();

	println!("⏳ Initializing subscriptions...\n");
	crowd_core_client::subscribe(&core_connection);
	local_account::subscribe(&core_connection);
	local_message::subscribe(&core_connection);
	core_connection.run_threaded();

	telegram_relay::subscribe(
		&core_connection,
		async_handler.clone(),
		telegram_relay_bot.clone(),
	);

	let telegram_relay_handler = dptree::entry()
		// .branch(
		// 	dptree::entry()
		// 		.filter_map(|update: telegram::Update| update.from().cloned())
		// 		.endpoint(telegram_user::handle_updates(core_connection.clone())),
		// )
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
					telegram_relay::handle_messages(core_connection.clone())
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
