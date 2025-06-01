use corvutils::StringExtensions;
use spacetimedb::{Identity, ReducerContext, ScheduleAt, Timestamp, reducer, table};

use crate::{
	common::ports::RecordResolver,
	domain::entities::shared::keys::{AccountId, ExternalActorId},
};

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

impl RecordResolver<ExternalAuthenticationRequest> for ExternalAuthenticationRequestId {
	fn try_resolve(&self, ctx: &ReducerContext) -> Result<ExternalAuthenticationRequest, String> {
		ctx.db
			.external_authentication_request()
			.id()
			.find(*self)
			.ok_or(format!("Account link request {self} does not exist."))
	}
}

#[table(
	name = external_authentication_request_schedule,
	scheduled(scheduled_delete_ext_auth_req)
)]
pub struct ExternalAuthenticationRequestExpirySchedule {
	#[primary_key]
	#[auto_inc]
	pub scheduled_id: u64,

	pub scheduled_at: ScheduleAt,
	pub request_id:   ExternalAuthenticationRequestId,
}

#[reducer]
/// Removes an expired external authentication request.
/// Should only be invoked via a scheduled task.
pub fn scheduled_delete_ext_auth_req(
	ctx: &ReducerContext, args: ExternalAuthenticationRequestExpirySchedule,
) -> Result<(), String> {
	if ctx.sender != ctx.identity() {
		return Err(r#"
			Reducer `scheduled_delete_external_authentication_request`
			may not be invoked by clients, only via scheduling.
		"#
		.to_string()
		.squash_whitespace());
	}

	ctx.db
		.external_authentication_request()
		.id()
		.delete(args.request_id);

	log::info!("Account link request {} expired.", args.request_id);

	Ok(())
}
