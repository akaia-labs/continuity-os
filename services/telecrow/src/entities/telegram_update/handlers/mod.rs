mod message;
mod user;

use std::{future::Future, pin::Pin, sync::Arc};

use corvutils::{print_error, print_success};
use crowdcomm::corvidx::DbConnection;
use teloxide::{
	RequestError, respond,
	types::{Update, UpdateKind},
};

use self::{message::on_message, user::on_user_update};
use crate::BotInstanceType;

pub fn root_handler(
	corvidx: Arc<DbConnection>,
) -> impl Fn(Update, BotInstanceType) -> Pin<Box<dyn Future<Output = Result<(), RequestError>> + Send>>
{
	move |update: Update, _bot: BotInstanceType| {
		let ctx = corvidx.clone();
		let user = update.from();

		if let Some(user) = user {
			let user_data = user.clone();

			Box::pin(async move {
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
