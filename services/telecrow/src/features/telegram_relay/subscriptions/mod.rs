mod events;
mod messages;

use std::sync::Arc;

use crowcomm::crowspace;
use teloxide::Bot;

use crate::common::runtime::AsyncHandler;

/// Aggregates all crowchat subscriptions
pub fn subscribe(
	crowspace_ctx: &crowspace::DbConnection, async_handler: Arc<AsyncHandler>, telegram_bot: Bot,
) {
	events::subscribe(&crowspace_ctx, async_handler.clone(), telegram_bot.clone());
	messages::subscribe(&crowspace_ctx, async_handler.clone(), telegram_bot.clone());
}
