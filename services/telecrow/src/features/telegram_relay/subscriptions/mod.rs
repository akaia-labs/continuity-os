mod events;
mod messages;

use std::sync::Arc;

use crate::common::{async_runtime, bindings::telegram};
use crowtocol_rs::crowchat;

/// Aggregates all crowchat subscriptions
pub fn subscribe(
	crowctx: &crowchat::DbConnection, async_handler: Arc<async_runtime::AsyncRuntime>,
	telegram_bot: telegram::Bot,
) {
	events::subscribe(&crowctx, async_handler.clone(), telegram_bot.clone());
	messages::subscribe(&crowctx, async_handler.clone(), telegram_bot.clone());
}
