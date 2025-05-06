mod events;
mod messages;

use std::sync::Arc;

use crowcomm::crowd_core::DbConnection;
use teloxide::Bot;

use crate::common::runtime::AsyncHandler;

/// Aggregates all crowchat subscriptions
pub fn subscribe(
	core_ctx: &DbConnection, async_handler: Arc<AsyncHandler>, telegram_bot: Bot,
) {
	events::subscribe(&core_ctx, async_handler.clone(), telegram_bot.clone());
	messages::subscribe(&core_ctx, async_handler.clone(), telegram_bot.clone());
}
