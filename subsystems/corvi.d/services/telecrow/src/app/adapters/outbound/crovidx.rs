use std::sync::Arc;

use crowdcomm_sdk::{runtime::AsyncHandler, singularity::stdb::DbConnection};

use crate::{
	BotInstanceType,
	domain::{entities::message, features::external_authentication},
};

/// Aggregates Singularity subscriptions
pub fn subscribe_to_singularity(
	telegram_bot: BotInstanceType, ctx: &DbConnection, async_handler: Arc<AsyncHandler>,
) {
	message::forward_to_telegram(&ctx, async_handler.clone(), telegram_bot.clone());
	external_authentication::forward_to_telegram(&ctx, async_handler, telegram_bot);
}
