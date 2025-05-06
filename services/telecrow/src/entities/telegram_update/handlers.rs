use std::{future::Future, pin::Pin, sync::Arc};

use crowcomm::crowd_core::DbConnection;
use teloxide::{
	Bot, RequestError, respond,
	types::{Update, UpdateKind},
};

use super::subscriptions::{on_message, on_user_update};

pub fn root_handler(
	core_ctx: Arc<DbConnection>,
) -> impl Fn(Update, Bot) -> Pin<Box<dyn Future<Output = Result<(), RequestError>> + Send>> {
	move |update: Update, _bot: Bot| {
		let ctx = core_ctx.clone();
		let user = update.from();

		if let Some(user) = user {
			let user_data = user.clone();

			Box::pin(async move {
				on_user_update(ctx.clone(), user_data);

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
