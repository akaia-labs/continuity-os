use spacetimedb::{Identity, Timestamp, table};

use crate::domain::entities::shared::keys::{ActorId, ChannelId};

pub type MessageId = i128;

#[table(name = message, public)]
pub struct Message {
	#[auto_inc]
	#[primary_key]
	pub id: MessageId,

	// TODO: Improve type safety by disallowing sending messages to superchannels
	#[index(btree)]
	pub channel: ChannelId,

	pub sent_at: Timestamp,
	pub sender:  Identity,

	#[index(btree)]
	/// The original message author.
	pub author: ActorId,

	pub text: String,
}
