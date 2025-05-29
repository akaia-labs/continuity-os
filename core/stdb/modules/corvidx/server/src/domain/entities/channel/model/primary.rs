use spacetimedb::{Timestamp, table};

use super::metadata::ChannelMetadata;
use crate::domain::entities::shared::keys::{AccountId, PrimaryChannelId, SubordinateChannelId};

#[table(name = primary_channel, public)]
/// A message channel that hosts other channels,
/// and does not hold member list, which is instead delegated to subchannels.
///
/// Addresses Matrix compatibility to some degree,
/// where it can be mapped to a `Room` with `"type": "m.space"`.
pub struct PrimaryChannel {
	#[primary_key]
	/// Maps to `opaque_id`: of `m.room.id`
	pub id: PrimaryChannelId,

	#[unique]
	#[index(btree)]
	/// Maps to #`localpart` of `m.room.canonical_alias`
	pub canonical_alias: String,

	#[index(btree)]
	pub creator: AccountId,

	// pub config:     ChannelConfigId,
	pub created_at: Timestamp,
	pub updated_at: Timestamp,

	/// Non-indexable additional properties.
	pub metadata: ChannelMetadata,

	pub subchannels: Vec<SubordinateChannelId>,
}
