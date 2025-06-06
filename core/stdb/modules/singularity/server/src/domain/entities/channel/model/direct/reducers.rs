use spacetimedb::{ReducerContext, Table, reducer};

use super::{DirectChannel, DirectChannelReference, direct_channel};
use crate::{common::ports::Resolvable, domain::entities::shared::keys::ActorId};

#[reducer]
/// Creates a new direct channel for the given actors `a` and `b`.
pub fn create_direct_channel(
	ctx: &ReducerContext, a_id: ActorId, b_id: ActorId,
) -> Result<(), String> {
	a_id.try_is_resolvable(ctx)?;
	b_id.try_is_resolvable(ctx)?;

	ctx.db
		.direct_channel()
		.try_insert(DirectChannel {
			id:         DirectChannelReference { a: a_id, b: b_id }.to_string(),
			creator:    ctx.sender,
			created_at: ctx.timestamp,
			updated_at: ctx.timestamp,
			messages:   vec![],
		})
		.map(|_| ())
		.map_err(|e| e.to_string())
}
