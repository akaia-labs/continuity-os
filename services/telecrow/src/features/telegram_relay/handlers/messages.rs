use std::{future::Future, pin::Pin, sync::Arc};

use crowcomm::{
	crowd_core::{DbConnection, account::ForeignAccountImport, import_message, send_message},
	telegram,
};
use teloxide::{Bot, RequestError, respond};

pub fn handle_messages(
	core_ctx: Arc<DbConnection>,
) -> impl Fn(telegram::Message, Bot) -> Pin<Box<dyn Future<Output = Result<(), RequestError>> + Send>>
{
	move |msg: telegram::Message, _bot: Bot| {
		let ctx = core_ctx.clone();

		Box::pin(async move {
			if let Some(text) = msg.text() {
				let _result = if let Some(author) = &msg.from {
					ctx.reducers
						.import_message(author.into_account_reference(), text.to_owned())
				} else {
					ctx.reducers.send_message(text.to_owned())
				};
			}

			respond(())
		})
	}
}
