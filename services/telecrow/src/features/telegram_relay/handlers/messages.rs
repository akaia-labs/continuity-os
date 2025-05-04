use std::{future::Future, pin::Pin};

use crowcomm::crowspace::send_message;
use teloxide::{Bot, RequestError, respond};

use crate::common::{bindings::telegram, clients::crowspace_client};

pub fn handle_message(
	crowspace_ctx: crowspace_client::ConnectionPointer,
) -> impl Fn(telegram::Message, Bot) -> Pin<Box<dyn Future<Output = Result<(), RequestError>> + Send>>
{
	move |msg: telegram::Message, _bot: Bot| {
		let crowspace_connection = crowspace_ctx.clone();

		Box::pin(async move {
			if let Some(text) = msg.text() {
				let _result = crowspace_connection.reducers.send_message(text.to_owned());
			}

			respond(())
		})
	}
}
