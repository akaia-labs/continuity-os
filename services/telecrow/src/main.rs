pub mod common;
pub mod entities;
pub mod features;

use crowtocol_rs::crowchat::{self, *};
use spacetimedb_sdk::{Table, TableWithPrimaryKey};

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

fn register_callbacks(crowctx: &crowchat::DbConnection) {
	crowctx
		.db
		.user()
		.on_insert(user_subscriptions::on_user_inserted);

	crowctx
		.reducers
		.on_set_name(user_subscriptions::on_name_set);

	crowctx
		.db
		.user()
		.on_update(user_subscriptions::on_user_updated);

	crowctx
		.reducers
		.on_send_message(message_subscriptions::on_message_sent);
}

#[tokio::main]
async fn main() -> Result<(), TelecrowError> {
	dotenvy::dotenv()?;
	pretty_env_logger::init();
	println!("Initializing connections...");

	let crowchat_connection = crowchat_client::connect();
	let telegram_bot_client = telegram::Bot::from_env();
	let runtime_service = async_runtime::create_service();

	crowchat_client::subscribe(&crowchat_connection);
	register_callbacks(&crowchat_connection);
	crowchat_connection.run_threaded();

	telegram_bridge::start_forwarding(
		telegram_bot_client.clone(),
		runtime_service.clone(),
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
