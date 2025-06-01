use spacetimedb::{Timestamp, table};

use super::metadata::ChannelMetadata;
use crate::domain::entities::shared::{
	keys::{AccountId, ActorId, PrimaryChannelId, SubordinateChannelId},
	message::MessageId,
};

#[table(name = subordinate_channel, public)]
/// A message channel confined within another, higher order channel.
///
/// Addresses Matrix compatibility to some degree,
/// where it can be mapped to a `Room` with `"type": "m.space.child"`.
pub struct SubordinateChannel {
	#[primary_key]
	/// Maps to `opaque_id`: of `m.room.id`
	pub id: SubordinateChannelId,

	#[unique]
	#[index(btree)]
	/// Maps to #`localpart` of `m.room.canonical_alias`
	pub canonical_alias: String,

	#[index(btree)]
	pub creator: AccountId,

	pub created_at:   Timestamp,
	pub updated_at:   Timestamp,
	// pub config:     ChannelConfigId,
	pub metadata:     ChannelMetadata,
	pub superchannel: PrimaryChannelId,
	pub members:      Vec<ActorId>,
	pub messages:     Vec<MessageId>,
}
