mod reducers;

use spacetimedb::{Timestamp, table};

use super::metadata::ChannelMetadata;
use crate::domain::entities::shared::{
	keys::{AccountId, ActorId, StandaloneChannelId},
	message::MessageId,
};

#[table(name = standalone_channel, public)]
/// A standalone message channel.
/// Useful for DMs or any other cases that do not require channel hierarchy.
///
/// Addresses Matrix compatibility to some degree,
/// where it can be mapped to a `Room` with `"type": null`.
pub struct StandaloneChannel {
	#[primary_key]
	/// Maps to the `opaque_id` part of `m.room.id`
	pub id: StandaloneChannelId,

	#[unique]
	#[index(btree)]
	/// Maps to #`localpart` of `m.room.canonical_alias`
	pub canonical_alias: String,

	#[index(btree)]
	pub creator: AccountId,

	pub created_at: Timestamp,
	pub updated_at: Timestamp,
	// pub config:  ChannelConfigId,
	pub metadata:   ChannelMetadata,
	pub members:    Vec<ActorId>,
	pub messages:   Vec<MessageId>,
}
