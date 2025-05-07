use std::sync::Arc;

use crowcomm::crowd_core::{
	DbConnection, ForeignAccountTableAccess,
	account::ForeignAccountImport,
	import_foreign_account, import_message,
	profile::{ProfileImport, ProfileRetrieval},
	send_message, update_foreign_account,
};
use teloxide::types::{Message, User};

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
