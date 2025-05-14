use spacetimedb::{Timestamp, table};

use crate::entities::native_account::NativeAccountId;

#[table(name = account_link_request, public)]
/// Represents a pending link invitation from a native Account to a foreign
/// Telegram account
pub struct AccountLinkRequest {
	#[primary_key]
	#[auto_inc]
	pub id:         i128,
	pub requester:  NativeAccountId,
	pub created_at: Timestamp,
	pub expires_at: Timestamp,
}
