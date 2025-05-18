mod corvidx_message;

use std::sync::Arc;

use crowdcomm_sdk::{corvidx::stdb::DbConnection, runtime::AsyncHandler};

use crate::{BotInstanceType, domain::features::account_linking};

/// Aggregates all corvidx subscriptions
pub fn subscribe(
	corvidx: &DbConnection, async_handler: Arc<AsyncHandler>, telegram_bot: BotInstanceType,
) {
	corvidx_message::subscribe(&corvidx, async_handler.clone(), telegram_bot.clone());
	// account_linking::subscribe(&corvidx, async_handler, telegram_bot);
}
