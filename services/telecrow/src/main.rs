mod app;
pub mod common;
pub mod domain;

use std::sync::Arc;

use crowdcomm_sdk::{
	configuration::corvid_subsystem_config::{self, CorvidSubsystemConfig},
	runtime::AsyncHandler,
};
use dotenvy::dotenv;
use teloxide::{
	Bot,
	adaptors::DefaultParseMode,
	dispatching::{HandlerExt, UpdateFilterExt},
	dptree,
	prelude::{Dispatcher, RequesterExt},
	types::{ParseMode, Update},
};

use crate::{common::clients::corvidx_client, domain::entities::command};

pub type TelecrowError = Box<dyn std::error::Error + Send + Sync>;
pub type BotInstanceType = DefaultParseMode<Bot>;

#[tokio::main]
async fn main() -> Result<(), TelecrowError> {
	pretty_env_logger::init();
	dotenv()?;

	let CorvidSubsystemConfig { components, .. } = corvid_subsystem_config::get();
	let async_handler = AsyncHandler::new();
	let corvidx_conn = Arc::new(corvidx_client::connect());

	let telegram_bridge_bot: BotInstanceType =
		Bot::new(components.telecrow.auth_token).parse_mode(ParseMode::Html);

	println!("⏳ Initializing subscriptions...\n");
	app::on_init(&corvidx_conn);

	app::subscribe_to_corvidx(
		telegram_bridge_bot.clone(),
		&corvidx_conn,
		async_handler.clone(),
	);

	let telegram_bridge_bot_handler = dptree::entry()
		.branch(
			Update::filter_callback_query()
				.endpoint(app::callback_query_handler(corvidx_conn.clone())),
		)
		.branch(
			Update::filter_message()
				.filter_command::<command::BasicCommand>()
				.endpoint(command::on_basic_command),
		)
		.branch(
			Update::filter_message()
				.filter_command::<command::PrivateCommand>()
				.endpoint(command::private_handler(corvidx_conn.clone())),
		)
		.branch(
			dptree::entry()
				.filter_map(|update: Update| update.from().cloned())
				.endpoint(app::telegram_update_handler(
					corvidx_conn.clone(),
					components.telecrow.delegated_authority_space_id,
				)),
		);

	println!("\n⏳ Initializing module clients...\n");
	corvidx_conn.run_threaded();

	println!("⌛ Starting Telegram bridge bot dispatcher...\n");
	Dispatcher::builder(telegram_bridge_bot, telegram_bridge_bot_handler)
		.build()
		.dispatch()
		.await;

	Ok(())
}
