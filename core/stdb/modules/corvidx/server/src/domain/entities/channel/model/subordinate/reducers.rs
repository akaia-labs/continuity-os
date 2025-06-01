use spacetimedb::{ReducerContext, Table, reducer};

use super::{
	super::{ChannelMetadata, PrimaryChannel},
	SubordinateChannel, subordinate_channel,
};
use crate::{
	common::{ports::RecordResolution, types::StUuid},
	domain::entities::shared::keys::{ActorId, ChannelId, ExternalActorId, PrimaryChannelId},
};

#[reducer]
/// Creates a new subordinate channel.
pub fn create_subordinate_channel(
	ctx: &ReducerContext, alias: String, metadata: Option<ChannelMetadata>,
	superchannel_id: PrimaryChannelId,
) -> Result<(), String> {
	let superchannel: PrimaryChannel =
		ChannelId::Primary(superchannel_id.clone()).try_resolve(ctx)?;

	ctx.db
		.subordinate_channel()
		.try_insert(SubordinateChannel {
			id:              StUuid::new(ctx).to_string(),
			canonical_alias: alias,
			creator:         ctx.sender,
			created_at:      ctx.timestamp,
			updated_at:      ctx.timestamp,
			metadata:        metadata.unwrap_or_default(),
			superchannel:    superchannel.id,
			members:         vec![ActorId::Internal(ctx.sender)],
			messages:        vec![],
		})
		.map(|_| ())
		.map_err(|e| e.to_string())
}

#[reducer]
/// Creates a record for an existing subchannel
/// bridged from an external source.
pub fn register_subordinate_channel(
	ctx: &ReducerContext, channel_id: String, alias: String, metadata: Option<ChannelMetadata>,
	superchannel_id: PrimaryChannelId, members: Option<Vec<ExternalActorId>>,
) -> Result<(), String> {
	let superchannel: PrimaryChannel =
		ChannelId::Primary(superchannel_id.clone()).try_resolve(ctx)?;

	ctx.db
		.subordinate_channel()
		.try_insert(SubordinateChannel {
			id:              channel_id,
			canonical_alias: alias,
			creator:         ctx.sender,
			created_at:      ctx.timestamp,
			updated_at:      ctx.timestamp,
			metadata:        metadata.unwrap_or_default(),
			superchannel:    superchannel.id,

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
		.map(|_| ())
		.map_err(|e| e.to_string())
}
