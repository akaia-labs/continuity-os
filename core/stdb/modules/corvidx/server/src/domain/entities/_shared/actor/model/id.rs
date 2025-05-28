use spacetimedb::SpacetimeType;

use super::{ExternalActorId, InternalActorId};

#[derive(SpacetimeType, Clone)]
pub enum ActorId {
	Internal(InternalActorId),
	External(ExternalActorId),

	// TODO: Consider removing in the future
	/// Fallback value, use with caution.
	Unknown,
}
