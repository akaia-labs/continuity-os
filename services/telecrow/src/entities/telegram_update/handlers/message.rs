use std::sync::Arc;

use crowdcomm::corvidx::{
	DbConnection, account::ForeignAccountImport, import_message, send_message,
};
use teloxide::types::Message;

pub fn on_message(core_ctx: Arc<DbConnection>, msg: Message) {
	if let Some(text) = msg.text() {
		let _result = if let Some(author) = &msg.from {
			core_ctx
				.reducers
				.import_message(author.into_account_reference(), text.to_owned())
		} else {
			core_ctx.reducers.send_message(text.to_owned())
		};
	}
}
