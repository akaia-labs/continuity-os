use spacetimedb::{ReducerContext, Table, reducer};

use super::{
	ChannelKind, ChannelMetadata, PrimaryChannel, StandaloneChannel, primary_channel,
	standalone_channel,
};
use crate::{common::types::StUuid, domain::entities::shared::actor::ActorId};

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
		members:         vec![ActorId::Internal(ctx.sender)],
		metadata:        metadata.unwrap_or_default(),
	});

	Ok(())
}

#[reducer]
/// Creates a record for an existing channel
/// bridged from an external source.
pub fn register_standalone_channel(
	ctx: &ReducerContext, id: String, alias: String, members: Option<Vec<ActorId>>,
	metadata: Option<ChannelMetadata>,
) -> Result<(), String> {
	ctx.db.standalone_channel().insert(StandaloneChannel {
		id,

		canonical_alias: alias,
		creator: ctx.sender,
		created_at: ctx.timestamp,
		updated_at: ctx.timestamp,
		members: vec![ActorId::Internal(ctx.sender)],
		metadata: metadata.unwrap_or_default(),
	});

	Ok(())
}
