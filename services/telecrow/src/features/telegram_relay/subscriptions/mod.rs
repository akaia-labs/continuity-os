mod events;
mod messages;

use std::sync::Arc;

use crowdcomm::crowd_core::DbConnection;

use crate::{BotInstanceType, common::runtime::AsyncHandler};

/// Aggregates all crowchat subscriptions
pub fn subscribe(
	core_ctx: &DbConnection, async_handler: Arc<AsyncHandler>, telegram_bot: BotInstanceType,
) {
	events::subscribe(&core_ctx, async_handler.clone(), telegram_bot.clone());
	messages::subscribe(&core_ctx, async_handler.clone(), telegram_bot.clone());
}
