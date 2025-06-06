mod reducers;

use spacetimedb::{ReducerContext, Timestamp, table};

use super::metadata::ChannelMetadata;
use crate::{
	common::ports::RecordResolver,
	domain::entities::shared::{
		keys::{AccountId, ActorId, ChannelId, StandaloneChannelId},
		message::MessageId,
	},
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

impl RecordResolver<StandaloneChannel> for ChannelId {
	fn try_resolve(&self, ctx: &ReducerContext) -> Result<StandaloneChannel, String> {
		match self {
			| ChannelId::Standalone(id) => ctx
				.db
				.standalone_channel()
				.id()
				.find(id)
				.ok_or(format!("Standalone channel {self} does not exist.")),

			| _ => Err(format!("Channel {self} is not a standalone channel.")),
		}
	}
}
