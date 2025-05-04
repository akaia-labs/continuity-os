use spacetimedb::{ReducerContext, Table, reducer};

use super::{tables::*, validation::*};
use crate::entities::{
	foreign_account::{ForeignAccountReference, foreign_account},
	local_account::local_account,
};

#[reducer]
/// Facilitates the basic internal messaging functionality
pub fn send_message(ctx: &ReducerContext, text: String) -> Result<(), String> {
	let author_id: MessageAuthorId =
		if let Some(author_account) = ctx.db.local_account().id().find(ctx.sender) {
			MessageAuthorId::LocalAccountId(author_account.id)
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
	ctx: &ReducerContext, author_reference: ForeignAccountReference, text: String,
) -> Result<(), String> {
	let author_id: MessageAuthorId = if let Some(author_foreign_account) = ctx
		.db
		.foreign_account()
		.id()
		.find(author_reference.to_string())
	{
		if let Some(author_account_id) = author_foreign_account.owner_id {
			MessageAuthorId::LocalAccountId(author_account_id)
		} else {
			MessageAuthorId::ForeignAccountId(author_foreign_account.id)
		}
	} else {
		MessageAuthorId::Unknown
	};

	let sender = match author_id {
		| MessageAuthorId::LocalAccountId(identity) => identity,
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
