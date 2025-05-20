mod event;
mod message;
mod user;

use std::{future::Future, pin::Pin, sync::Arc};

use corvutils::{print_error, print_success};
use crowdcomm_sdk::corvidx::stdb::DbConnection;
use teloxide::{
	RequestError, respond,
	types::{Update, UpdateKind},
};

use self::{event::on_unauthorized_use_attempt, message::on_message, user::on_user_update};
use crate::BotInstanceType;

pub fn root_handler(
	ctx: Arc<DbConnection>, delegated_authority_groupchat_id: String,
) -> impl Fn(Update, BotInstanceType) -> Pin<Box<dyn Future<Output = Result<(), RequestError>> + Send>>
{
	move |update: Update, _bot: BotInstanceType| {
		let ctx = ctx.clone();

		let is_origin_authorized = update
			.chat()
			.and_then(|chat| {
				if chat.is_group() || chat.is_supergroup() {
					Some(chat.id.to_string() == delegated_authority_groupchat_id)
				} else {
					Some(chat.is_private())
				}
			})
			.unwrap_or(false);

		if !is_origin_authorized {
			return Box::pin(async move {
				on_unauthorized_use_attempt(ctx.clone(), update);
				respond(())
			});
		}

		let user = update.from();

		if let Some(user) = user {
			let user_data = user.clone();

			Box::pin(async move {
				// ! CRITICAL:
				// TODO!: Only handle user updates emitted after bot's initialization
				on_user_update(ctx.clone(), user_data, print_success, print_error);

				match update.kind {
					| UpdateKind::Message(msg) => on_message(ctx.clone(), msg),
					| _ => {},
				}

				respond(())
			})
		} else {
			Box::pin(async move { respond(()) })
		}
	}
}
