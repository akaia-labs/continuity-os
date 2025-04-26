use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use crowtocol_rs::crowchat::{self, send_message};
use teloxide::{Bot, RequestError, respond};

use crate::common::bindings::telegram;

pub fn handle_message(
	crowchat_connection: Arc<crowchat::DbConnection>,
) -> impl Fn(telegram::Message, Bot) -> Pin<Box<dyn Future<Output = Result<(), RequestError>> + Send>>
{
	move |msg: telegram::Message, _bot: Bot| {
		let connection = crowchat_connection.clone();

		Box::pin(async move {
			if let Some(text) = msg.text() {
				let _ = connection.reducers.send_message(text.to_owned());
			}

			respond(())
		})
	}
}
