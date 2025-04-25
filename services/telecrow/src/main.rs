mod common;
pub mod entities;
pub mod features;

use crowtocol_rs::crowchat::{self, *};
use spacetimedb_sdk::{Table, TableWithPrimaryKey};
use tokio::sync::mpsc;

use common::{
	bindings::telegram::{self, *},
	clients::crowchat_client,
	runtime::{self, RuntimeService},
};
use std::sync::Arc;

use entities::{message_subscriptions, user_subscriptions};
use features::message_forwarding;

pub type TelecrowError = Box<dyn std::error::Error + Send + Sync>;

/// Registers all the callbacks the app will use to respond to database events.
fn register_callbacks(
	crowctx: &crowchat::DbConnection,
	forward_transmitter: mpsc::Sender<message_forwarding::TelegramForwardRequest>,
	runtime: Arc<RuntimeService>,
) {
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
		.db
		.message()
		.on_insert(message_subscriptions::handle_telegram_forward(
			forward_transmitter,
			runtime,
		));

	crowctx
		.reducers
		.on_send_message(message_subscriptions::on_message_sent);
}

fn on_tg_text_message(crowctx: &crowchat::DbConnection, tg_message: telegram::Message) {
	if let Some(text) = tg_message.text() {
		crowctx.reducers.send_message(text.to_owned()).unwrap();
	}
}

async fn process_text_message(
	_tg_bot: telegram::Bot, tg_user: telegram::User, message_text: String,
) -> Result<(), TelecrowError> {
	println!(
		"@{}: {}",
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
	println!("Initializing connections...");

	let crowctx = crowchat_client::connect();
	let telegram_bot_client = telegram::Bot::from_env();

	// Create the runtime service
	let runtime_service = runtime::create_service();

	// Channel for forwarding messages to Telegram
	let (forward_transmitter, mut forward_receiver) =
		mpsc::channel::<message_forwarding::TelegramForwardRequest>(100);

	// Telegram bot instance for the background task
	let telegram_transmitter = telegram_bot_client.clone();

	// Spawn a background task that processes messages from the channel
	runtime_service.handle().spawn(async move {
		while let Some(req) = forward_receiver.recv().await {
			let _ = telegram_transmitter
				.send_message(
					telegram::ChatId(req.chat_id),
					format!("💬 {}: {}", req.sender_name, req.message_text),
				)
				.message_thread_id(ThreadId(telegram::MessageId(3315)))
				.await;
		}
	});

	crowchat_client::subscribe(&crowctx);
	register_callbacks(&crowctx, forward_transmitter, runtime_service);
	crowctx.run_threaded();

	let teloxide_schema = telegram::Update::filter_message()
	.inspect(move |msg: telegram::Message| on_tg_text_message(&crowctx, msg))
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
		telegram::Message::filter_text().endpoint(process_text_message),
	);

	println!("Starting Telegram bot client...");

	telegram::Dispatcher::builder(telegram_bot_client, teloxide_schema)
		.build()
		.dispatch()
		.await;

	Ok(())
}
