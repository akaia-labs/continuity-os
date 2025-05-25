use std::sync::Arc;

use corvutils::StringExtensions;
use crowdcomm_sdk::{
	corvidx::stdb::{DbConnection, send_message},
	integrations::ports::ExternalActorIdentification,
	presentation::Summary,
};
use teloxide::types::Update;

use crate::common::constants::SERVICE_CANONICAL_NAME;

pub fn handle_unauthorized_use_attempt(corvidx: Arc<DbConnection>, event: Update) {
	let initiator_specifier = event
		.from()
		.map(|user| user.into_exref().to_string())
		.unwrap_or("an unknown user".into());

	let log_header =
		format!("Unauthorized {SERVICE_CANONICAL_NAME} use attempt by {initiator_specifier}.");

	let chat_summary = event.chat().map(|chat| chat.summary().padded());

	let log_text = chat_summary.map_or_else(
		|| log_header.clone(),
		|summary| format!("{log_header}\n{summary}"),
	);

	let _result = corvidx.reducers.send_message(log_text);
}
