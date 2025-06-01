use spacetimedb::{ReducerContext, Table, reducer};

use super::{super::ChannelMetadata, StandaloneChannel, standalone_channel};
use crate::{
	common::types::StUuid,
	domain::entities::shared::keys::{ActorId, ExternalActorId},
};

#[reducer]
/// Creates a new standalone channel.
pub fn create_standalone_channel(
	ctx: &ReducerContext, alias: String, metadata: Option<ChannelMetadata>,
) -> Result<(), String> {
	ctx.db.standalone_channel().insert(StandaloneChannel {
		id:              StUuid::new(ctx).to_string(),
		canonical_alias: alias,
		creator:         ctx.sender,
		created_at:      ctx.timestamp,
		updated_at:      ctx.timestamp,
		metadata:        metadata.unwrap_or_default(),
		members:         vec![ActorId::Internal(ctx.sender)],
		messages:        vec![],
	});

	Ok(())
}

#[reducer]
/// Creates a record for an existing channel
/// bridged from an external source.
pub fn register_standalone_channel(
	ctx: &ReducerContext, channel_id: String, alias: String, members: Option<Vec<ExternalActorId>>,
	metadata: Option<ChannelMetadata>,
) -> Result<(), String> {
	ctx.db.standalone_channel().insert(StandaloneChannel {
		id:              channel_id,
		canonical_alias: alias,
		creator:         ctx.sender,
		created_at:      ctx.timestamp,
		updated_at:      ctx.timestamp,
		metadata:        metadata.unwrap_or_default(),

		members: members
			.map(|ext_ids| {
				ext_ids
					.iter()
					.map(|id| ActorId::External(id.clone()))
					.collect()
			})
			.unwrap_or_default(),

		messages: vec![],
	});

	Ok(())
}
