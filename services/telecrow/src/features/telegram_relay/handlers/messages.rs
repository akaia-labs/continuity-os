use std::{future::Future, pin::Pin, sync::Arc};

use crowcomm::{
	crowd_core::{DbConnection, send_message},
	telegram,
};
use teloxide::{Bot, RequestError, respond};

pub fn handle_messages(
	core_ctx: Arc<DbConnection>,
) -> impl Fn(telegram::Message, Bot) -> Pin<Box<dyn Future<Output = Result<(), RequestError>> + Send>>
{
	move |msg: telegram::Message, _bot: Bot| {
		let crowspace_connection = core_ctx.clone();

		Box::pin(async move {
			if let Some(text) = msg.text() {
				let _result = crowspace_connection.reducers.send_message(text.to_owned());
			}

			respond(())
		})
	}
}
