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
	features::telegram_bridge,
};

#[tokio::main]
async fn main() -> Result<(), runtime::TelecrowError> {
	dotenv()?;
	pretty_env_logger::init();
	println!("Initializing connections...");

	let async_runtime_instance = async_runtime::new();
	let crowchat_connection = crowchat_client::connect();
	let crowchat_connection_pointer = Arc::new(crowchat_connection);
	let telegram_bot_client = telegram::Bot::from_env();

	crowchat_client::subscribe(&crowchat_connection_pointer);
	crowchat_user::subscribe(&crowchat_connection_pointer);
	crowchat_message::subscribe(&crowchat_connection_pointer);
	crowchat_connection_pointer.run_threaded();

	telegram_bridge::capture_crowchat_events(
		telegram_bot_client.clone(),
		async_runtime_instance.clone(),
		&crowchat_connection_pointer,
	);

	telegram_bridge::capture_crowchat_messages(
		telegram_bot_client.clone(),
		async_runtime_instance.clone(),
		&crowchat_connection_pointer,
	);

	let message_handler = move |msg: telegram::Message, _bot: telegram::Bot| {
		let connection = crowchat_connection_pointer.clone();
		async move {
			if let Some(text) = msg.text() {
				let _ = connection.reducers.send_message(text.to_owned());
			}
			respond(())
		}
	};

	let teloxide_schema = telegram::Update::filter_message()
		.branch(
			dptree::entry()
			.filter_command::<telegram_bridge::BasicCommand>()
			.endpoint(telegram_bridge::on_basic_command),
		)
		/*
		   Inject the `User` object representing the author of an incoming
		   message into every successive handler function (1)
		*/
		.filter_map(|update: telegram::Update| update.from().cloned())
		.branch(
			dptree::endpoint(message_handler),
		);

	println!("Starting Telegram bot client...");

	telegram::Dispatcher::builder(telegram_bot_client, teloxide_schema)
		.build()
		.dispatch()
		.await;

	Ok(())
}
