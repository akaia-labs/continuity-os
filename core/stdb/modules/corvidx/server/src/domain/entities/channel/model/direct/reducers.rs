use spacetimedb::{ReducerContext, Table, reducer};

use super::{DirectChannel, direct_channel};
use crate::common::types::StUuid;

#[reducer]
/// Creates a new primary channel.
pub fn create_direct_channel(ctx: &ReducerContext) -> Result<(), String> {
	ctx.db
		.direct_channel()
		.try_insert(DirectChannel {
			id:         StUuid::new(ctx).to_string(),
			creator:    ctx.sender,
			created_at: ctx.timestamp,
			updated_at: ctx.timestamp,
		})
		.map(|_| ())
		.map_err(|e| e.to_string())
}

#[reducer]
/// Creates a record for an existing channel space
/// bridged from an external source.
pub fn register_direct_channel(ctx: &ReducerContext, channel_id: String) -> Result<(), String> {
	ctx.db
		.direct_channel()
		.try_insert(DirectChannel {
			id:         channel_id,
			creator:    ctx.sender,
			created_at: ctx.timestamp,
			updated_at: ctx.timestamp,
		})
		.map(|_| ())
		.map_err(|e| e.to_string())
}
