use spacetimedb::{ReducerContext, Table, reducer};

use super::{
	super::{ChannelMetadata, PrimaryChannel},
	SubordinateChannel, subordinate_channel,
};
use crate::{
	common::{ports::RecordResolution, types::StUuid},
	domain::entities::{
		channel::primary_channel,
		shared::{
			keys::{ActorId, ChannelId, ExternalActorId, PrimaryChannelId, SubordinateChannelId},
			message::message,
		},
	},
};

#[reducer]
/// Creates a new subordinate channel.
pub fn create_subchannel(
	ctx: &ReducerContext, alias: String, metadata: Option<ChannelMetadata>,
	superchannel_id: PrimaryChannelId,
) -> Result<(), String> {
	let mut superchannel: PrimaryChannel =
		ChannelId::Primary(superchannel_id.clone()).try_resolve(ctx)?;

	let subchannel = ctx
		.db
		.subordinate_channel()
		.try_insert(SubordinateChannel {
			id:              StUuid::new(ctx).to_string(),
			canonical_alias: alias,
			creator:         ctx.sender,
			created_at:      ctx.timestamp,
			updated_at:      ctx.timestamp,
			metadata:        metadata.unwrap_or_default(),
			superchannel:    superchannel_id,
			members:         vec![ActorId::Internal(ctx.sender)],
			messages:        vec![],
		})
		.map_err(|e| e.to_string())?;

	superchannel.subchannels.push(subchannel.id);
	ctx.db.primary_channel().id().update(superchannel);

	Ok(())
}

#[reducer]
/// Creates a record for an existing subchannel
/// bridged from an external source.
pub fn register_subchannel(
	ctx: &ReducerContext, subchannel_id: String, alias: String, metadata: Option<ChannelMetadata>,
	superchannel_id: PrimaryChannelId, members: Option<Vec<ExternalActorId>>,
) -> Result<(), String> {
	let mut superchannel: PrimaryChannel =
		ChannelId::Primary(superchannel_id.clone()).try_resolve(ctx)?;

	let subchannel = ctx
		.db
		.subordinate_channel()
		.try_insert(SubordinateChannel {
			id:              subchannel_id,
			canonical_alias: alias,
			creator:         ctx.sender,
			created_at:      ctx.timestamp,
			updated_at:      ctx.timestamp,
			metadata:        metadata.unwrap_or_default(),
			superchannel:    superchannel_id,

			members: members
				.map(|ext_ids| {
					ext_ids
						.iter()
						.map(|id| ActorId::External(id.clone()))
						.collect()
				})
				.unwrap_or_default(),

			messages: vec![],
		})
		.map_err(|e| e.to_string())?;

	superchannel.subchannels.push(subchannel.id);
	ctx.db.primary_channel().id().update(superchannel);

	Ok(())
}

#[reducer]
/// Deletes a subchannel along with all of its messages, if any.
pub fn delete_subchannel(ctx: &ReducerContext, id: SubordinateChannelId) -> Result<(), String> {
	let channel: SubordinateChannel = ChannelId::Subordinate(id).try_resolve(ctx)?;

	for message_id in channel.messages {
		ctx.db.message().id().delete(&message_id);
	}

	ctx.db.subordinate_channel().id().delete(channel.id);

	Ok(())
}
