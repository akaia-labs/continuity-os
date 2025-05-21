use std::{future::Future, pin::Pin, sync::Arc};

use crowdcomm_sdk::{
	corvidx::stdb::DbConnection,
	integrations::{
		commands::AlrActionResolution,
		dtos::{ActionDescriptor, ActionKind, ActionResolutionCommand},
	},
};
use teloxide::{RequestError, prelude::Requester, respond, types::CallbackQuery};

use crate::{BotInstanceType, domain::features::account_linking};

pub fn callback_query_handler(
	ctx: Arc<DbConnection>,
) -> impl Fn(
	BotInstanceType,
	CallbackQuery,
) -> Pin<Box<dyn Future<Output = Result<(), RequestError>> + Send>> {
	move |bot: BotInstanceType, callback_query: CallbackQuery| {
		let ctx = ctx.clone();

		let action_descriptor = callback_query
			.data
			.as_ref()
			.map(|d| ActionDescriptor::try_from_str(d.as_str()).ok())
			.flatten();

		if action_descriptor.is_none() || callback_query.data.is_none() {
			return Box::pin(async move {
				bot.answer_callback_query(&callback_query.id).await?;
				respond(())
			});
		}

		let ActionDescriptor { kind: action_kind } = action_descriptor.unwrap();
		let query_payload = callback_query.data.unwrap();

		match action_kind {
			| ActionKind::AccountLinkRequest => {
				let command: Result<ActionResolutionCommand<AlrActionResolution>, String> =
					ActionResolutionCommand::try_from_str(query_payload.as_str());

				if let Ok(command) = command {
					return Box::pin(async move {
						account_linking::handle_request_callback(ctx, bot.clone(), command);
						respond(())
					});
				}
			},
		}

		Box::pin(async move {
			bot.answer_callback_query(&callback_query.id).await?;
			respond(())
		})
	}
}
