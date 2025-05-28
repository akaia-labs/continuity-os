use spacetimedb::SpacetimeType;

use crate::domain::entities::shared::keys::{AccountId, ExternalActorId};

#[derive(SpacetimeType, Clone)]
pub enum ActorId {
	Internal(AccountId),
	External(ExternalActorId),

	// TODO: Consider removing in the future
	/// Fallback value, use with caution.
	Unknown,
}
