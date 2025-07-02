use spacetimedb::{ReducerContext, Table, reducer};

use crate::{
	common::ports::RecordResolver,
	domain::entities::{
		external_actor::ExternalActorReference,
		shared::{
			keys::{ActorId, ChannelId},
			message::{Message, message, validate_message},
		},
	},
};

#[reducer]
/// Facilitates the basic internal messaging functionality
pub fn send_message(
	ctx: &ReducerContext, channel_id: ChannelId, text: String,
) -> Result<(), String> {
	let author = ctx.sender.try_resolve(ctx)?;

	let text = validate_message(text)?;

	log::info!("{}", text);

	ctx.db.message().insert(Message {
		id: 0,
		channel: channel_id,
		sender: ctx.sender,
		sent_at: ctx.timestamp,
		author: ActorId::Internal(author.id),
		text,
	});

	Ok(())
}

#[reducer]
// Registers a message relayed from an external platform
pub fn import_message(
	ctx: &ReducerContext, channel_id: ChannelId, author_ref: ExternalActorReference, text: String,
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
		channel: channel_id,
		sender,
		sent_at: ctx.timestamp,
		author: ActorId::External(author.id),
		text,
	});

	Ok(())
}
