mod corvidx_events;
mod corvidx_messages;

use std::sync::Arc;

use crowdcomm_sdk::corvidx::stdb::DbConnection;

use crate::{BotInstanceType, common::runtime::AsyncHandler};

/// Aggregates all corvidx subscriptions
pub fn subscribe(
	corvidx: &DbConnection, async_handler: Arc<AsyncHandler>, telegram_bot: BotInstanceType,
) {
	corvidx_events::subscribe(&corvidx, async_handler.clone(), telegram_bot.clone());
	corvidx_messages::subscribe(&corvidx, async_handler.clone(), telegram_bot.clone());
}
