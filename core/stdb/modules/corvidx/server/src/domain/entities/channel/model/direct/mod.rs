mod reducers;

use std::fmt::Display;

use spacetimedb::{ReducerContext, SpacetimeType, Timestamp, table};

use crate::{
	common::ports::RecordResolution,
	domain::entities::shared::keys::{AccountId, ActorId, ChannelId, DirectChannelId},
};

#[table(name = direct_channel, public)]
/// A message channel
pub struct DirectChannel {
	#[primary_key]
	pub id: DirectChannelId,

	#[index(btree)]
	pub creator: AccountId,

	pub created_at: Timestamp,
	pub updated_at: Timestamp,
}

#[derive(SpacetimeType, Clone)]
pub struct DirectChannelReference {
	pub a: ActorId,
	pub b: ActorId,
}

impl RecordResolution<DirectChannel> for DirectChannelId {
	fn try_resolve(&self, ctx: &ReducerContext) -> Result<DirectChannel, String> {
		ctx.db
			.direct_channel()
			.id()
			.find(self)
			.ok_or(format!("Primary channel {self} does not exist."))
	}
}

impl RecordResolution<DirectChannel> for ChannelId {
	fn try_resolve(&self, ctx: &ReducerContext) -> Result<DirectChannel, String> {
		match self {
			| ChannelId::Direct(id) => id.try_resolve(ctx),
			| _ => Err(format!("Channel {self} is not a primary channel.")),
		}
	}
}

impl Display for DirectChannelReference {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}<>{}", self.a, self.b)
	}
}
