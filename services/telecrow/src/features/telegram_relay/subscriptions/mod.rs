mod events;
mod messages;

use std::sync::Arc;

use crate::common::async_runtime;
use crowtocol_rs::crowchat;
use teloxide::Bot;

/// Aggregates all crowchat subscriptions
pub fn subscribe(
	crowctx: &crowchat::DbConnection, async_handler: Arc<async_runtime::AsyncRuntime>,
	telegram_bot: Bot,
) {
	events::subscribe(&crowctx, async_handler.clone(), telegram_bot.clone());
	messages::subscribe(&crowctx, async_handler.clone(), telegram_bot.clone());
}
