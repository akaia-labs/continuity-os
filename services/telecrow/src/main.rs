pub mod application;
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
	types::{CallbackQuery, InlineQuery, ParseMode, Update},
};

use crate::{
	application::{general_subscriptions, telegram_bridge},
	common::clients::corvidx_client,
	domain::entities::{telegram_command, telegram_update},
};

pub type TelecrowError = Box<dyn std::error::Error + Send + Sync>;
pub type BotInstanceType = DefaultParseMode<Bot>;

#[tokio::main]
async fn main() -> Result<(), TelecrowError> {
	pretty_env_logger::init();
	dotenv()?;

	let CorvidSubsystemConfig { components, .. } = corvid_subsystem_config::get();
	let async_handler = AsyncHandler::new();
	let corvidx_connection = Arc::new(corvidx_client::connect());

	let telegram_bridge_bot: BotInstanceType =
		Bot::new(components.telecrow.auth_token).parse_mode(ParseMode::Html);

	println!("⏳ Initializing subscriptions...\n");
	general_subscriptions::init(&corvidx_connection);

	telegram_bridge::subscribe(
		&corvidx_connection,
		async_handler.clone(),
		telegram_bridge_bot.clone(),
	);

	let telegram_bridge_bot_handler = dptree::entry()
		.branch(Update::filter_inline_query().endpoint(
			async |_bot: BotInstanceType, iq: InlineQuery| {
				println!("Received inline query: {}", iq.query);

				Ok(())
			},
		))
		.branch(Update::filter_callback_query().endpoint(
			async |_bot: BotInstanceType, cq: CallbackQuery| {
				println!(
					"Received callback query: {}",
					cq.data.unwrap_or("".to_string())
				);

				Ok(())
			},
		))
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
				.endpoint(telegram_update::root_handler(
					corvidx_connection.clone(),
					components.telecrow.delegated_authority_space_id,
				)),
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
