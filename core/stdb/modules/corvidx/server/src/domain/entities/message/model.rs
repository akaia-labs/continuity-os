use spacetimedb::{Identity, Timestamp, table};

use crate::domain::entities::shared::actor::ActorId;

pub type MessageId = i128;

#[table(name = message, public)]
pub struct Message {
	#[auto_inc]
	#[primary_key]
	pub id: MessageId,

	pub sent_at: Timestamp,
	pub sender:  Identity,

	#[index(btree)]
	/// The original message author.
	pub author_id: ActorId,

	pub text: String,
}
