mod common;
pub mod entities;

use crowlink::clients::crownest::{self, *};
use spacetimedb_sdk::{Table, TableWithPrimaryKey};
use tokio::sync::mpsc;

use common::clients::{
	crownest_client,
	telegram_bot_client::{self, *},
};

use entities::{message_subscriptions, user_subscriptions};

pub type TelecrowError = Box<dyn std::error::Error + Send + Sync>;

// Message structure for the channel to forward Telegram messages
pub struct TelegramForwardRequest {
	sender_name: String,
	message_text: String,
	chat_id: i64,
}

/// Registers all the callbacks the app will use to respond to database events.
fn register_callbacks(crowctx: &crownest::DbConnection, tx: mpsc::Sender<TelegramForwardRequest>) {
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
		.on_insert(message_subscriptions::handle_telegram_forward(tx));

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
	println!(
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

	println!("Initializing DB connection...");
	let crowctx = crownest_client::connect();

	println!("Initializing Telegram bot...");
	let telegram_bot = telegram_bot_client::Bot::from_env();

	// Create a channel for forwarding messages to Telegram
	let (tx, mut rx) = mpsc::channel::<TelegramForwardRequest>(100);

	// Clone the bot for the background task
	let tg_bot_clone = telegram_bot.clone();

	// Spawn a background task that processes messages from the channel
	tokio::spawn(async move {
		while let Some(req) = rx.recv().await {
			let _ = tg_bot_clone
				.send_message(
					telegram_bot_client::ChatId(req.chat_id),
					format!("@{}: {}", req.sender_name, req.message_text),
				)
				.await;
		}
	});

	register_callbacks(&crowctx, tx);
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

	println!("Starting Telegram bot client...");
	telegram_bot_client::Dispatcher::builder(telegram_bot, teloxide_schema)
		.build()
		.dispatch()
		.await;

	Ok(())
}
