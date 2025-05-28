use spacetimedb::{SpacetimeType, Timestamp, table};

use crate::domain::entities::shared::actor::{ActorId, InternalActorId};

pub type ChannelId = String;

#[table(name = channel, public)]
/// A message channel.
/// Addresses Matrix compatibility to some degree,
/// where it can be mapped to `Room`.
pub struct Channel {
	#[primary_key]
	/// Maps to `opaque_id`: of `m.room.id`
	pub id: ChannelId,

	#[unique]
	#[index(btree)]
	/// Maps to #`localpart` of `m.room.canonical_alias`
	pub canonical_alias: String,

	#[index(btree)]
	pub creator: InternalActorId,

	// pub config:     ChannelConfigId,
	pub created_at: Timestamp,
	pub updated_at: Timestamp,

	/// Non-indexable additional properties.
	pub metadata: ChannelMetadata,

	pub members: Vec<ActorId>,

	pub subchannels: Option<Vec<ChannelId>>,
}

#[derive(SpacetimeType, Clone)]
pub enum ChannelKind {
	/// Maps to `"type": null` in Matrix
	Default,

	/// Maps to `"type": "m.space"` in Matrix
	SuperChannel,
}
