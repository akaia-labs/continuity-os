mod reducers;

use std::{
	fmt::{self, Display},
	str::FromStr,
};

use spacetimedb::{ReducerContext, SpacetimeType, Timestamp, table};

use crate::{
	common::ports::RecordResolver,
	domain::entities::shared::{
		keys::{AccountId, ActorId, ChannelId, DirectChannelId},
		message::MessageId,
	},
};

#[table(name = direct_channel, public,)]
/// A message channel
pub struct DirectChannel {
	#[primary_key]
	pub id: DirectChannelId,

	#[index(btree)]
	pub creator: AccountId,

	pub created_at: Timestamp,
	pub updated_at: Timestamp,
	pub messages:   Vec<MessageId>,
}

impl RecordResolver<DirectChannel> for ChannelId {
	fn try_resolve(&self, ctx: &ReducerContext) -> Result<DirectChannel, String> {
		match self {
			| ChannelId::Direct(id) => id.try_resolve(ctx),
			| _ => Err(format!("Channel {self} is not a direct channel.")),
		}
	}
}

impl RecordResolver<DirectChannel> for DirectChannelId {
	fn try_resolve(&self, ctx: &ReducerContext) -> Result<DirectChannel, String> {
		ctx.db
			.direct_channel()
			.id()
			.find(self)
			.ok_or(format!("Direct channel {self} does not exist."))
	}
}

#[derive(SpacetimeType, Clone)]
/// A reference to the direct channel shared between two actors.
pub struct DirectChannelReference {
	pub a: ActorId,
	pub b: ActorId,
}

impl DirectChannelReference {
	pub const DELIMITER: &str = "<>";
}

impl Display for DirectChannelReference {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let mut pair: Vec<String> = vec![
			self.a.to_string().to_lowercase(),
			self.b.to_string().to_lowercase(),
		];

		pair.sort();
		write!(f, "{}{}{}", pair[0], Self::DELIMITER, pair[1])
	}
}

pub type DirectChannelReferenceParseErr = &'static str;

impl FromStr for DirectChannelReference {
	type Err = DirectChannelReferenceParseErr;

	/// Converts a direct channel id into a direct channel reference.
	///
	/// Note that this won't validate external actor ids.
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut parts = s.rsplitn(2, Self::DELIMITER);
		let a_str = parts.next().ok_or("invalid channel id")?;
		let b_str = parts.next().ok_or("invalid channel id")?;

		let a = a_str.parse::<AccountId>().map_or_else(
			|_| ActorId::External(a_str.to_owned()),
			|a| ActorId::Internal(a),
		);

		let b = b_str.parse::<AccountId>().map_or_else(
			|_| ActorId::External(b_str.to_owned()),
			|b| ActorId::Internal(b),
		);

		Ok(DirectChannelReference { a, b })
	}
}
