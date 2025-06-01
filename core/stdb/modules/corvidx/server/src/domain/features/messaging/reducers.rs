use spacetimedb::{ReducerContext, Table, reducer};

use crate::{
	common::ports::RecordResolution,
	domain::entities::{
		external_actor::ExternalActorReference,
		shared::{
			keys::ActorId,
			message::{Message, message, validate_message},
		},
	},
};

#[reducer]
/// Facilitates the basic internal messaging functionality
pub fn send_message(ctx: &ReducerContext, text: String) -> Result<(), String> {
	let author_id: ActorId = if let Some(account) = ctx.sender.try_resolve(ctx).ok() {
		ActorId::Internal(account.id)
	} else {
		ActorId::Unknown
	};

	let text = validate_message(text)?;

	log::info!("{}", text);

	ctx.db.message().insert(Message {
		id: 0,
		sender: ctx.sender,
		sent_at: ctx.timestamp,
		author: author_id,
		text,
	});

	Ok(())
}

#[reducer]
// Registers a message relayed from an external platform
pub fn import_message(
	ctx: &ReducerContext, author_ref: ExternalActorReference, text: String,
) -> Result<(), String> {
	let author = author_ref.try_resolve(ctx)?;

	let sender = if let Some(identity) = author.account {
		identity
	} else {
		ctx.sender
	};

	let text = validate_message(text)?;

	ctx.db.message().insert(Message {
		id: 0,
		sender,
		sent_at: ctx.timestamp,
		author: ActorId::External(author.id),
		text,
	});

	Ok(())
}
