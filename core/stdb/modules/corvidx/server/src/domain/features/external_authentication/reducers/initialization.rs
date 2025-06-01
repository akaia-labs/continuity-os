use std::time::Duration;

use spacetimedb::{ReducerContext, Table, reducer};

use crate::{
	common::ports::RecordResolution,
	domain::entities::{
		action_request::{
			ExternalAuthenticationRequest, ExternalAuthenticationRequestExpirySchedule,
			external_authentication_request, external_authentication_request_schedule,
		},
		external_actor::ExternalActorReference,
	},
};

const LINK_REQUEST_TIMEOUT: Duration = Duration::from_secs(5 * 60);

// TODO Implement rate limit
#[reducer]
/// Creates an external authentication request.
pub fn initiate_external_authentication(
	ctx: &ReducerContext, ext_actor_ref: ExternalActorReference,
) -> Result<(), String> {
	let external_actor = ext_actor_ref.try_resolve(ctx)?;

	if external_actor.account.is_some() {
		return Err(format!(
			"External actor {ext_actor_ref} is already linked to another account.",
		));
	}

	let account = ctx.sender.try_resolve(ctx)?;

	let request = ctx
		.db
		.external_authentication_request()
		.insert(ExternalAuthenticationRequest {
			id:         0,
			issuer:     ctx.identity(),
			created_at: ctx.timestamp,
			requester:  account.id,
			subject:    external_actor.id,

			expires_at: ctx
				.timestamp
				.checked_add(LINK_REQUEST_TIMEOUT.into())
				.ok_or(format!(
					"Unable to calculate account link request expiration date for {ext_actor_ref}."
				))?,
		});

	ctx.db.external_authentication_request_schedule().insert(
		ExternalAuthenticationRequestExpirySchedule {
			scheduled_id: 0,
			scheduled_at: request.expires_at.into(),
			request_id:   request.id,
		},
	);

	log::info!(
		"{requester} created an account link request {id} for third-party account {ext_actor_ref}.",
		requester = account.id,
		id = request.id,
	);

	Ok(())
}
