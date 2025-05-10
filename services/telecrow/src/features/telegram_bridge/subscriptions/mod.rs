mod events;
mod messages;

use std::sync::Arc;

use crowdcomm::corvidx::DbConnection;

use crate::{BotInstanceType, common::runtime::AsyncHandler};

/// Aggregates all crowchat subscriptions
pub fn subscribe(
	corvidx: &DbConnection, async_handler: Arc<AsyncHandler>, telegram_bot: BotInstanceType,
) {
	events::subscribe(&corvidx, async_handler.clone(), telegram_bot.clone());
	messages::subscribe(&corvidx, async_handler.clone(), telegram_bot.clone());
}
