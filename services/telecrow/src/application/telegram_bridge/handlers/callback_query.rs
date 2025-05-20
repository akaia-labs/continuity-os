use std::{future::Future, pin::Pin, sync::Arc};

use corvutils::{print_error, print_success};
use crowdcomm_sdk::{
	corvidx::stdb::DbConnection,
	integrations::{
		commands::AlrActionResolution,
		dtos::{ActionDescriptor, ActionKind, ActionResolutionCommand},
	},
};
use teloxide::{RequestError, prelude::Requester, respond, types::CallbackQuery};

use crate::BotInstanceType;

pub fn root_handler(
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

		if let Some(ActionDescriptor { kind: action_kind }) = action_descriptor {
			match action_kind {
				| ActionKind::AccountLinkRequest => {
					if let Some(payload) = callback_query.data {
						let alr_response: Result<
							ActionResolutionCommand<AlrActionResolution>,
							String,
						> = ActionResolutionCommand::try_from_str(payload.as_str());

						if let Ok(alr_response) = alr_response {
							match alr_response.resolution {
								| AlrActionResolution::Accept(id) => {
									// let alr = corvidx_conn.db.account_link_request().id().find(&
									// id) 	.ok_or(format!("Account link request {id} does
									// not exist."));

									println!("Account link request {id} has been accepted.");
								},

								| AlrActionResolution::Reject(id) => {
									println!("Account link request {id} has been rejected.");
								},
							}
						}
					}
				},
			}
		}

		Box::pin(async move {
			bot.answer_callback_query(&callback_query.id).await?;

			respond(())
		})
	}
}
