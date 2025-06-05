use std::sync::Arc;

use crowdcomm_sdk::{corvidx::stdb::DbConnection, runtime::AsyncHandler};

use crate::{
	BotInstanceType,
	domain::{entities::message, features::external_authentication},
};

/// Aggregates corvidx subscriptions
pub fn subscribe_to_corvidx(
	telegram_bot: BotInstanceType, ctx: &DbConnection, async_handler: Arc<AsyncHandler>,
) {
	message::forward_to_telegram(&ctx, async_handler.clone(), telegram_bot.clone());
	external_authentication::forward_to_telegram(&ctx, async_handler, telegram_bot);
}
