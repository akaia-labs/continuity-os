use std::pin::Pin;

use teloxide::{Bot, RequestError, respond};

use crate::common::{bindings::telegram, clients::crowspace_client};

pub fn handle_update(
	crowspace_ctx: crowspace_client::ConnectionPointer,
) -> impl Fn(telegram::User, Bot) -> Pin<Box<dyn Future<Output = Result<(), RequestError>> + Send>>
{
	move |msg: telegram::User, _bot: Bot| {
		let csctx = crowspace_ctx.clone();

		Box::pin(async move {
			if let Some(text) = msg.text() {
				let _result = csctx.reducers.send_message(text.to_owned());
			}

			respond(())
		})
	}
}
