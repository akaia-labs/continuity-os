pub mod common;
pub mod entities;
pub mod features;

use std::sync::Arc;

use crowcomm::telegram;
use dotenvy::dotenv;
use entities::{telegram_command, telegram_update};
use teloxide::{
	Bot,
	adaptors::DefaultParseMode,
	dispatching::{HandlerExt, UpdateFilterExt},
	dptree,
	prelude::{Dispatcher, RequesterExt},
	types::ParseMode,
};

use crate::{
	common::{clients::crowd_core_client, runtime, runtime::TelecrowError},
	entities::{local_account, local_message},
	features::telegram_relay,
};

pub type BotInstanceType = DefaultParseMode<Bot>;

#[tokio::main]
async fn main() -> Result<(), TelecrowError> {
	dotenv()?;
	pretty_env_logger::init();
	println!("\n⏳ Initializing clients...\n");

	let async_handler = runtime::new_async_handler();
	let core_connection = Arc::new(crowd_core_client::connect());
	let telegram_bridge: BotInstanceType = Bot::from_env().parse_mode(ParseMode::MarkdownV2);

	println!("⏳ Initializing subscriptions...\n");
	crowd_core_client::subscribe(&core_connection);
	local_account::subscribe(&core_connection);
	local_message::subscribe(&core_connection);
	core_connection.run_threaded();

	telegram_relay::subscribe(
		&core_connection,
		async_handler.clone(),
		telegram_bridge.clone(),
	);

	let telegram_relay_handler = dptree::entry()
		.branch(
			telegram::Update::filter_message()
				.filter_command::<telegram_command::BasicCommand>()
				.endpoint(telegram_command::on_basic_command),
		)
		.branch(
			telegram::Update::filter_message()
				.filter_command::<telegram_command::UserCommand>()
				.endpoint(telegram_command::user_handler(core_connection.clone())),
		)
		.branch(
			dptree::entry()
				.filter_map(|update: telegram::Update| update.from().cloned())
				.endpoint(telegram_update::root_handler(core_connection.clone())),
		);

	println!("⌛ Starting Telegram bot dispatcher...\n");

	Dispatcher::builder(telegram_bridge, telegram_relay_handler)
		.build()
		.dispatch()
		.await;

	Ok(())
}
