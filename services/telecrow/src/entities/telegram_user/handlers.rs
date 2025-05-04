use std::{pin::Pin, sync::Arc};

use crowcomm::{crowd_core::DbConnection, telegram};
use teloxide::{Bot, RequestError, respond};

pub fn handle_updates(
	core_ctx: Arc<DbConnection>,
) -> impl Fn(telegram::User, Bot) -> Pin<Box<dyn Future<Output = Result<(), RequestError>> + Send>>
{
	move |msg: telegram::User, _bot: Bot| {
		let csctx = core_ctx.clone();

		Box::pin(async move {
			if let Some(text) = msg.text() {
				let _result = csctx.reducers.send_message(text.to_owned());
			}

			respond(())
		})
	}
}
