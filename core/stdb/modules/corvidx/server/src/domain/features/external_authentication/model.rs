use spacetimedb::{Identity, ScheduleAt, Timestamp, table};

use super::reducers::scheduled_delete_external_authentication_request;
use crate::domain::entities::shared::keys::{AccountId, ExternalActorId};

pub type ExternalAuthenticationRequestId = u64;

#[table(name = external_authentication_request, public)]
/// Represents a pending link request
/// from an internal account to a third-party platform actor
pub struct ExternalAuthenticationRequest {
	#[primary_key]
	#[auto_inc]
	pub id: ExternalAuthenticationRequestId,

	pub issuer:     Identity,
	pub requester:  AccountId,
	pub subject:    ExternalActorId,
	pub created_at: Timestamp,
	pub expires_at: Timestamp,
}

#[table(
	name = external_authentication_request_schedule,
	scheduled(scheduled_delete_external_authentication_request)
)]
pub struct ExternalAuthenticationRequestExpirySchedule {
	#[primary_key]
	#[auto_inc]
	pub scheduled_id: u64,

	pub scheduled_at: ScheduleAt,
	pub request_id:   ExternalAuthenticationRequestId,
}
