mod reducers;

use spacetimedb::{ReducerContext, Timestamp, table};

pub use self::reducers::delete_subchannel;
use super::metadata::ChannelMetadata;
use crate::{
	common::ports::RecordResolution,
	domain::entities::shared::{
		keys::{AccountId, ActorId, ChannelId, PrimaryChannelId, SubordinateChannelId},
		message::MessageId,
	},
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

	pub created_at: Timestamp,
	pub updated_at: Timestamp,
	// pub config:  ChannelConfigId,
	pub metadata:   ChannelMetadata,

	#[index(btree)]
	pub superchannel: PrimaryChannelId,

	pub members:  Vec<ActorId>,
	pub messages: Vec<MessageId>,
}

impl RecordResolution<SubordinateChannel> for ChannelId {
	fn try_resolve(&self, ctx: &ReducerContext) -> Result<SubordinateChannel, String> {
		match self {
			| ChannelId::Subordinate(id) => ctx
				.db
				.subordinate_channel()
				.id()
				.find(id)
				.ok_or(format!("Subordinate channel {self} does not exist.")),

			| _ => Err(format!("Channel {self} is not a subordinate channel.")),
		}
	}
}
