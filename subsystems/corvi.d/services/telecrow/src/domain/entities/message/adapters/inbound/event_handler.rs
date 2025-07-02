use std::sync::Arc;

use crowdcomm_sdk::{
	integrations::ports::ExternalActorIdentification,
	singularity::stdb::{ChannelId, DbConnection, import_message, send_message},
};
use teloxide::types::Message;

pub fn handle_telegram_message(ctx: Arc<DbConnection>, msg: Message) {
	if let Some(text) = msg.text() {
		let chat_id = msg.chat.id;
		let channel_id = ChannelId::Standalone(chat_id.to_string());

		let _result = if let Some(author) = &msg.from {
			ctx.reducers
				.import_message(channel_id, author.into_actor_ref(), text.to_owned())
		} else {
			ctx.reducers.send_message(channel_id, text.to_owned())
		};
	}
}
