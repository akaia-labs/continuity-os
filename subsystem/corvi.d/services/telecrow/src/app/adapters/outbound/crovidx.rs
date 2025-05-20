use std::sync::Arc;

use crowdcomm_sdk::{corvidx::stdb::DbConnection, runtime::AsyncHandler};

use crate::{
	BotInstanceType,
	domain::{entities::message, features::account_linking},
};

/// Aggregates corvidx subscriptions
pub fn subscribe_to_corvidx(
	telegram_bot: BotInstanceType, ctx: &DbConnection, async_handler: Arc<AsyncHandler>,
) {
	message::forward_to_telegram(&ctx, async_handler.clone(), telegram_bot.clone());
	account_linking::forward_to_telegram(&ctx, async_handler, telegram_bot);
}
