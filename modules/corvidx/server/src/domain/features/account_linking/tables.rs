use spacetimedb::{ScheduleAt, Timestamp, table};

use super::reducers::scheduled_delete_account_link_request;
use crate::domain::entities::{tp_account::TpAccountId, native_account::NativeAccountId};

pub type AccountLinkRequestId = i128;

#[table(name = account_link_request, public)]
/// Represents a pending link request
/// from a native account to a third-party account
pub struct AccountLinkRequest {
	#[primary_key]
	#[auto_inc]
	pub id: AccountLinkRequestId,

	pub created_at:           Timestamp,
	pub expires_at:           Timestamp,
	pub requester_account_id: NativeAccountId,
	pub subject_account_id:   TpAccountId,
}

#[table(name = account_link_request_schedule, scheduled(scheduled_delete_account_link_request))]
pub struct AccountLinkRequestExpirySchedule {
	#[primary_key]
	#[auto_inc]
	pub scheduled_id: u64,

	pub scheduled_at: ScheduleAt,
	pub request_id:   AccountLinkRequestId,
}
