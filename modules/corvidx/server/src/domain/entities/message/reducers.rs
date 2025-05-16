use capitalize::Capitalize;
use spacetimedb::{ReducerContext, Table, reducer};

use super::{tables::*, validation::*};
use crate::entities::{
	foreign_account::{ForeignAccountReference, foreign_account},
	native_account::native_account,
};

#[reducer]
/// Facilitates the basic internal messaging functionality
pub fn send_message(ctx: &ReducerContext, text: String) -> Result<(), String> {
	let author_id: MessageAuthorId =
		if let Some(author_account) = ctx.db.native_account().id().find(ctx.sender) {
			MessageAuthorId::NativeAccountId(author_account.id)
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
	let ForeignAccountReference {
		id: external_author_id,
		platform_tag,
	} = author_reference.clone();

	let author_account = ctx
		.db
		.foreign_account()
		.id()
		.find(author_reference.to_string())
		.ok_or(format!(
			"{platform_name} account {external_author_id} is not registered in the system.",
			platform_name = platform_tag.to_string().capitalize(),
		))?;

	let sender = if author_account.owner_id != ctx.identity() {
		author_account.owner_id
	} else {
		ctx.sender
	};

	let text = validate_message(text)?;

	ctx.db.message().insert(Message {
		id: 0,
		sender,
		sent_at: ctx.timestamp,
		author_id: MessageAuthorId::ForeignAccountId(author_account.id),
		text,
	});

	Ok(())
}
