use spacetimedb::{Identity, SpacetimeType, Timestamp, table};

use crate::entities::{foreign_account::ForeignAccountId, native_account::NativeAccountId};

#[derive(SpacetimeType)]
/// The original message author.
pub enum MessageAuthorId {
	System,
	NativeAccountId(NativeAccountId),
	ForeignAccountId(ForeignAccountId),
	/// Fallback value, use with caution.
	Unknown,
}

#[table(name = message, public)]
pub struct Message {
	#[auto_inc]
	#[primary_key]
	pub id:        i128,
	pub sent_at:   Timestamp,
	pub sender:    Identity,
	#[index(btree)]
	pub author_id: MessageAuthorId,
	pub text:      String,
}
