use spacetimedb::{Timestamp, table};

use crate::entities::{foreign_account::ForeignAccountId, native_account::NativeAccountId};

#[table(name = account_link_request, public)]
/// Represents a pending link request
/// from a native account to a foreign account
pub struct AccountLinkRequest {
	#[primary_key]
	#[auto_inc]
	pub id:                   i128,
	pub created_at:           Timestamp,
	pub expires_at:           Timestamp,
	pub requester_account_id: NativeAccountId,
	pub subject_account_id:   ForeignAccountId,
}
