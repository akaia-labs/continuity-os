pub mod common;
pub mod entities;
pub mod features;

use dotenvy::dotenv;
use std::sync::Arc;

use teloxide::{
	Bot,
	dispatching::{HandlerExt, UpdateFilterExt},
	dptree,
	prelude::Dispatcher,
};

use crate::{
	common::{async_runtime, bindings::telegram, clients::crowchat_client, runtime::TelecrowError},
	entities::{crowchat_message, crowchat_user},
	features::telegram_relay,
};

#[tokio::main]
async fn main() -> Result<(), TelecrowError> {
	dotenv()?;
	pretty_env_logger::init();
	println!("\n⏳ Initializing clients...\n");

	let async_runtime_instance = async_runtime::new();
	let crowchat_connection = Arc::new(crowchat_client::connect());
	let telegram_bot_client = Bot::from_env();

	println!("⏳ Initializing subscriptions...\n");
	crowchat_client::subscribe(&crowchat_connection);
	crowchat_user::subscribe(&crowchat_connection);
	crowchat_message::subscribe(&crowchat_connection);
	crowchat_connection.run_threaded();

	telegram_relay::subscribe(
		&crowchat_connection,
		async_runtime_instance.clone(),
		telegram_bot_client.clone(),
	);

	let telegram_relay_handler = telegram::Update::filter_message()
		.branch(
			dptree::entry()
			.filter_command::<telegram_relay::BasicCommand>()
			.endpoint(telegram_relay::on_basic_command),
		)
		// Injecting the `User` object representing the author of an incoming message
		.filter_map(|update: telegram::Update| update.from().cloned())
		.branch(
			dptree::endpoint(telegram_relay::handle_message(crowchat_connection.clone())),
		);

	println!("⌛ Starting Telegram bot dispatcher...\n");

	Dispatcher::builder(telegram_bot_client, telegram_relay_handler)
		.build()
		.dispatch()
		.await;

	Ok(())
}
