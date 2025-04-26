pub mod common;
pub mod entities;
pub mod features;

use crowtocol_rs::crowchat::*;
use dotenvy::dotenv;
use std::sync::Arc;

use crate::{
	common::{
		async_runtime,
		bindings::telegram::{self, *},
		clients::crowchat_client,
		runtime,
	},
	entities::{crowchat_message, crowchat_user},
	features::telegram_relay,
};

#[tokio::main]
async fn main() -> Result<(), runtime::TelecrowError> {
	dotenv()?;
	pretty_env_logger::init();
	println!("\n⏳ Initializing clients...\n");

	let async_runtime_instance = async_runtime::new();
	let crowchat_connection = Arc::new(crowchat_client::connect());
	let telegram_bot_client = telegram::Bot::from_env();

	println!("⏳ Initializing subscriptions...\n");
	crowchat_client::subscribe(&crowchat_connection);
	crowchat_user::subscribe(&crowchat_connection);
	crowchat_message::subscribe(&crowchat_connection);
	crowchat_connection.run_threaded();

	telegram_relay::capture_crowchat_events(
		telegram_bot_client.clone(),
		async_runtime_instance.clone(),
		&crowchat_connection,
	);

	telegram_relay::capture_crowchat_messages(
		telegram_bot_client.clone(),
		async_runtime_instance.clone(),
		&crowchat_connection,
	);

	let message_handler = move |msg: telegram::Message, _bot: telegram::Bot| {
		let connection = crowchat_connection.clone();
		async move {
			if let Some(text) = msg.text() {
				let _ = connection.reducers.send_message(text.to_owned());
			}
			respond(())
		}
	};

	let telegram_traffic_handler = telegram::Update::filter_message()
		.branch(
			dptree::entry()
			.filter_command::<telegram_relay::BasicCommand>()
			.endpoint(telegram_relay::on_basic_command),
		)
		/*
		   Inject the `User` object representing the author of an incoming
		   message into every successive handler function (1)
		*/
		.filter_map(|update: telegram::Update| update.from().cloned())
		.branch(
			dptree::endpoint(message_handler),
		);

	println!("⌛ Starting Telegram bot dispatcher...\n");

	telegram::Dispatcher::builder(telegram_bot_client, telegram_traffic_handler)
		.build()
		.dispatch()
		.await;

	Ok(())
}
