use std::{future::Future, pin::Pin, sync::Arc};

use crowcomm::crowspace::{self, send_message};
use teloxide::{Bot, RequestError, respond};

use crate::common::bindings::telegram;

pub fn handle_message(
	stdb: Arc<crowspace::DbConnection>,
) -> impl Fn(telegram::Message, Bot) -> Pin<Box<dyn Future<Output = Result<(), RequestError>> + Send>>
{
	move |msg: telegram::Message, _bot: Bot| {
		let crowchat_connection = stdb.clone();

		Box::pin(async move {
			if let Some(text) = msg.text() {
				let _result = crowchat_connection.reducers.send_message(text.to_owned());
			}

			respond(())
		})
	}
}
