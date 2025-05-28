use spacetimedb::{ReducerContext, Table, reducer};

use super::{model::*, validation::*};
use crate::{
	common::ports::RecordResolution,
	domain::entities::{external_actor::ExternalActorReference, shared::actor::ActorId},
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
		author_id,
		text,
	});

	Ok(())
}

#[reducer]
// Registers a message relayed from an external platform
pub fn import_message(
	ctx: &ReducerContext, ext_author_ref: ExternalActorReference, text: String,
) -> Result<(), String> {
	let actor = ext_author_ref.try_resolve(ctx)?;

	let sender = if let Some(identity) = actor.account {
		identity
	} else {
		ctx.sender
	};

	let text = validate_message(text)?;

	ctx.db.message().insert(Message {
		id: 0,
		sender,
		sent_at: ctx.timestamp,
		author_id: ActorId::External(actor.id),
		text,
	});

	Ok(())
}
