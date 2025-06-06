use std::sync::Arc;

use crowdcomm_sdk::{
	singularity::stdb::{DbConnection, import_message, send_message},
	integrations::ports::ExternalActorIdentification,
};
use teloxide::types::Message;

pub fn handle_telegram_message(ctx: Arc<DbConnection>, msg: Message) {
	if let Some(text) = msg.text() {
		let _result = if let Some(author) = &msg.from {
			ctx.reducers
				.import_message(author.into_actor_ref(), text.to_owned())
		} else {
			ctx.reducers.send_message(text.to_owned())
		};
	}
}
