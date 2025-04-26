use std::sync::Arc;

use crowtocol_rs::crowchat::{self, send_message};
use teloxide::{Bot, RequestError, respond};

use crate::common::bindings::telegram;

pub fn handle_message(
	crowchat_connection: Arc<crowchat::DbConnection>,
) -> impl Fn(telegram::Message, Bot) -> impl Future<Output = Result<(), RequestError>> {
	move |msg: telegram::Message, _bot: telegram::Bot| {
		let connection = crowchat_connection.clone();

		async move {
			if let Some(text) = msg.text() {
				let _ = connection.reducers.send_message(text.to_owned());
			}
			respond(())
		}
	}
}
