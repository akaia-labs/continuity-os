pub mod common;
pub mod entities;
pub mod features;

use crate::{
	common::{
		async_runtime,
		bindings::telegram::{self, *},
		clients::crowchat_client,
		runtime::*,
	},
	entities::{message_subscriptions, user_subscriptions},
	features::telegram_bridge,
};

#[tokio::main]
async fn main() -> Result<(), TelecrowError> {
	dotenvy::dotenv()?;
	pretty_env_logger::init();
	println!("Initializing connections...");

	let async_runtime_instance = async_runtime::new();
	let crowchat_connection = crowchat_client::connect();
	let telegram_bot_client = telegram::Bot::from_env();

	crowchat_client::subscribe(&crowchat_connection);
	user_subscriptions::register_internal_callbacks(&crowchat_connection);
	message_subscriptions::register_internal_callbacks(&crowchat_connection);
	crowchat_connection.run_threaded();

	telegram_bridge::event_capture_init(
		telegram_bot_client.clone(),
		async_runtime_instance.clone(),
		&crowchat_connection,
	);

	telegram_bridge::message_capture_init(
		telegram_bot_client.clone(),
		async_runtime_instance.clone(),
		&crowchat_connection,
	);

	let teloxide_schema = telegram::Update::filter_message()
		.inspect(move |msg: telegram::Message| message_subscriptions::on_tg_message_received(
			&crowchat_connection, msg
		))
		/*
		   Inject the `User` object representing the author of an incoming
		   message into every successive handler function (1)
		*/
		.filter_map(|update: telegram::Update| update.from().cloned())
		.branch(
			/*
			   Use filter_text method of MessageFilterExt to accept
			   only textual messages. Others will be ignored by this handler (2)
			*/
			telegram::Message::filter_text().endpoint(message_subscriptions::process_text_message),
		);

	println!("Starting Telegram bot client...");

	telegram::Dispatcher::builder(telegram_bot_client, teloxide_schema)
		.build()
		.dispatch()
		.await;

	Ok(())
}
