use spacetimedb::{Identity, SpacetimeType, Timestamp, table};

use crate::domain::entities::{account::AccountId, external_actor::ExternalActorId};

#[derive(SpacetimeType)]
/// The original message author.
pub enum MessageAuthorId {
	AccountId(AccountId),
	ExternalActorId(ExternalActorId),
	/// Fallback value, use with caution.
	Unknown,
}

#[table(name = message, public)]
pub struct Message {
	#[auto_inc]
	#[primary_key]
	pub id: i128,

	pub sent_at: Timestamp,
	pub sender:  Identity,

	#[index(btree)]
	pub author_id: MessageAuthorId,

	pub text: String,
	// TODO: track message forwarding
	// pub forwarded_to: Vec<ExternalChannelId>
}
