use spacetimedb::{SpacetimeType, Timestamp, table};

use crate::domain::entities::shared::actor::{ActorId, InternalActorId};

pub type ChannelId = String;

#[table(name = channel, public)]
/// A standalone message channel.
/// Addresses Matrix compatibility to some degree,
/// where it can be mapped to a `Room` with `"type": null`.
pub struct StandaloneChannel {
	#[primary_key]
	/// Maps to the `opaque_id` part of `m.room.id`
	pub id: ChannelId,

	#[unique]
	#[index(btree)]
	/// Maps to #`localpart` of `m.room.canonical_alias`
	pub canonical_alias: String,

	#[index(btree)]
	pub creator: InternalActorId,

	// pub config:  ChannelConfigId,
	pub created_at: Timestamp,
	pub updated_at: Timestamp,

	/// Non-indexable additional properties.
	pub metadata: ChannelMetadata,

	pub members: Vec<ActorId>,
}
