use spacetimedb::{ReducerContext, Table, reducer};

use super::{tables::*, validation::*};
use crate::entities::{
	external_account::{ExternalAccountReference, external_account},
	internal_account::account,
};

#[reducer]
/// Facilitates the basic internal messaging functionality
pub fn send_message(ctx: &ReducerContext, text: String) -> Result<(), String> {
	let author_id: MessageAuthorId =
		if let Some(author_account) = ctx.db.account().id().find(ctx.sender) {
			MessageAuthorId::InternalAccountId(author_account.id)
		} else {
			MessageAuthorId::System
		};

	let text = validate_message(text)?;

	log::info!("{}", text);

	ctx.db.message().insert(Message {
		id: 0,
		sender: ctx.sender,
		sent_at: ctx.timestamp,
		author_id,
		text,
	});

	Ok(())
}

#[reducer]
// Registers a message relayed from an external platform
pub fn import_message(
	ctx: &ReducerContext, author_reference: ExternalAccountReference, text: String,
) -> Result<(), String> {
	let author_id: MessageAuthorId = if let Some(author_external_account) = ctx
		.db
		.external_account()
		.id()
		.find(author_reference.to_string())
	{
		if let Some(author_account_id) = author_external_account.owner_id {
			MessageAuthorId::InternalAccountId(author_account_id)
		} else {
			MessageAuthorId::ExternalAccountId(author_external_account.id)
		}
	} else {
		MessageAuthorId::Unknown
	};

	let sender = match author_id {
		| MessageAuthorId::InternalAccountId(identity) => identity,
		| _ => ctx.sender,
	};

	let text = validate_message(text)?;

	log::info!("{}", text);

	ctx.db.message().insert(Message {
		id: 0,
		sender,
		sent_at: ctx.timestamp,
		author_id,
		text,
	});

	Ok(())
}
