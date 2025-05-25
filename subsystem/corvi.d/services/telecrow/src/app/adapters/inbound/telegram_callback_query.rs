use std::{future::Future, pin::Pin, sync::Arc};

use crowdcomm_sdk::{
	corvidx::stdb::DbConnection,
	integrations::{
		commands::AccountLinkRequestAction,
		dtos::{ActionCommand, ActionDescriptor, ActionKind},
	},
};
use teloxide::{RequestError, prelude::Requester, respond, types::CallbackQuery};

use crate::{BotInstanceType, domain::features::external_authentication};

pub fn callback_query_handler(
	ctx: Arc<DbConnection>,
) -> impl Fn(
	BotInstanceType,
	CallbackQuery,
) -> Pin<Box<dyn Future<Output = Result<(), RequestError>> + Send>> {
	move |bot: BotInstanceType, callback_query: CallbackQuery| {
		let ctx = ctx.clone();
		let prompt_msg = callback_query.message;
		let caller = callback_query.from;

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
				let command: Result<ActionCommand<AccountLinkRequestAction>, String> =
					ActionCommand::try_from_str(query_payload.as_str());

				if let Ok(command) = command {
					return Box::pin(async move {
						bot.answer_callback_query(&callback_query.id).await?;

						external_authentication::handle_command(
							ctx,
							bot.clone(),
							prompt_msg,
							command,
							caller,
						)
						.await;

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
