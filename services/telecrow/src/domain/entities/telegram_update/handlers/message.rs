use std::sync::Arc;

use crowdcomm_sdk::{
	corvidx::stdb::{DbConnection, import_message, send_message},
	integrations::TpAccountImport,
};
use teloxide::types::Message;

pub fn on_message(corvidx: Arc<DbConnection>, msg: Message) {
	if let Some(text) = msg.text() {
		let _result = if let Some(author) = &msg.from {
			corvidx
				.reducers
				.import_message(author.into_account_reference(), text.to_owned())
		} else {
			corvidx.reducers.send_message(text.to_owned())
		};
	}
}
