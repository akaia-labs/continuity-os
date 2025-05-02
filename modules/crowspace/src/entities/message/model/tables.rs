use spacetimedb::{Identity, SpacetimeType, Timestamp, table};

use crate::entities::{account::AccountId, external_account::ExternalAccountId};

#[derive(SpacetimeType)]
/// The original message author.
pub enum MessageAuthorId {
	System,
	InternalAccountId(AccountId),
	ExternalAccountId(ExternalAccountId),
	/// Fallback value, use with caution.
	Unknown,
}

#[table(name = message, public)]
pub struct Message {
	#[auto_inc]
	#[primary_key]
	pub id: u64,
	pub sent_at: Timestamp,
	pub sender: Identity,
	#[index(btree)]
	pub author_id: MessageAuthorId,
	pub text: String,
}
