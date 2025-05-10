pub mod common;
pub mod entities;
pub mod features;

use std::sync::Arc;

use crowdcomm::corvid_subsystem_config::{self, CorvidSubsystemConfig};
use dotenvy::dotenv;
use entities::{telegram_command, telegram_update};
use teloxide::{
	Bot,
	adaptors::DefaultParseMode,
	dispatching::{HandlerExt, UpdateFilterExt},
	dptree,
	prelude::{Dispatcher, RequesterExt},
	types::{ParseMode, Update},
};

use crate::{
	common::{clients::corvidx_client, runtime, runtime::TelecrowError},
	entities::{corvidx_account, corvidx_message},
	features::telegram_group_bridge,
};

pub type BotInstanceType = DefaultParseMode<Bot>;

#[tokio::main]
async fn main() -> Result<(), TelecrowError> {
	pretty_env_logger::init();
	dotenv()?;

	let CorvidSubsystemConfig { components, .. } = corvid_subsystem_config::get();
	let async_handler = runtime::new_async_handler();
	let corvidx_connection = Arc::new(corvidx_client::connect());

	let telegram_bridge_bot: BotInstanceType =
		Bot::new(components.telecrow.auth_token).parse_mode(ParseMode::Html);

	println!("⏳ Initializing subscriptions...\n");
	corvidx_client::subscribe(&corvidx_connection);
	corvidx_account::subscribe(&corvidx_connection);
	corvidx_message::subscribe(&corvidx_connection);

	telegram_group_bridge::subscribe(
		&corvidx_connection,
		async_handler.clone(),
		telegram_bridge_bot.clone(),
	);

	let telegram_bridge_bot_handler = dptree::entry()
		.branch(
			Update::filter_message()
				.filter_command::<telegram_command::BasicCommand>()
				.endpoint(telegram_command::on_basic_command),
		)
		.branch(
			Update::filter_message()
				.filter_command::<telegram_command::PrivateCommand>()
				.endpoint(telegram_command::private_handler(
					corvidx_connection.clone(),
				)),
		)
		.branch(
			dptree::entry()
				.filter_map(|update: Update| update.from().cloned())
				.endpoint(telegram_update::root_handler(corvidx_connection.clone())),
		);

	println!("\n⏳ Initializing module clients...\n");
	corvidx_connection.run_threaded();

	println!("⌛ Starting Telegram bridge bot dispatcher...\n");
	Dispatcher::builder(telegram_bridge_bot, telegram_bridge_bot_handler)
		.build()
		.dispatch()
		.await;

	Ok(())
}
