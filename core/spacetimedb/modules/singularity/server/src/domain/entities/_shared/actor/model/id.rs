use spacetimedb::ReducerContext;

use crate::{common::ports::Resolvable, domain::entities::shared::keys::ActorId};

impl Resolvable for ActorId {
	fn try_is_resolvable(&self, ctx: &ReducerContext) -> Result<(), String> {
		match self {
			| ActorId::Internal(id) => id.try_is_resolvable(ctx),
			| ActorId::External(id) => id.try_is_resolvable(ctx),
		}
	}
}
