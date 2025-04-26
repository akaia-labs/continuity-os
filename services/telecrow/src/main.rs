pub mod common;
pub mod entities;
pub mod features;

use crowtocol_rs::crowchat::{self, *};

use crate::{
	common::{
		async_runtime,
		bindings::telegram::{self, *},
		clients::crowchat_client,
		runtime,
	},
	entities::{message_subscriptions, user_subscriptions},
	features::telegram_bridge,
};

#[tokio::main]
async fn main() -> Result<(), runtime::TelecrowError> {
	dotenvy::dotenv()?;
	pretty_env_logger::init();
	println!("Initializing connections...");

	let async_runtime_instance = async_runtime::new();
	let crowchat_connection = crowchat_client::connect();
	// Create a static reference to the crowchat connection that can be shared
	let connection_arc = std::sync::Arc::new(crowchat_connection);
	let telegram_bot_client = telegram::Bot::from_env();

	// Use direct reference for initial setup
	crowchat_client::subscribe(&connection_arc);
	user_subscriptions::register_internal_callbacks(&connection_arc);
	message_subscriptions::register_internal_callbacks(&connection_arc);
	connection_arc.run_threaded();

	// telegram_bridge::Command::repl(telegram_bot_client.clone(), telegram_bridge::on_command).await;

	telegram_bridge::event_capture_init(
		telegram_bot_client.clone(),
		async_runtime_instance.clone(),
		&connection_arc,
	);

	telegram_bridge::message_capture_init(
		telegram_bot_client.clone(),
		async_runtime_instance.clone(),
		&connection_arc,
	);

	let message_handler = move |msg: telegram::Message, _bot: telegram::Bot| {
		let connection = connection_arc.clone();
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
