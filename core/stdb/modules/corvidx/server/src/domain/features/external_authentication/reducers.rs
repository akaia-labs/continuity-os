use std::time::Duration;

use capitalize::Capitalize;
use corvutils::StringExtensions;
use spacetimedb::{ReducerContext, Table, reducer};

use super::model::{
	ExternalAuthenticationRequest, ExternalAuthenticationRequestExpirySchedule,
	ExternalAuthenticationRequestId, external_authentication_request,
};
use crate::{
	common::ports::RecordResolution,
	domain::{
		entities::{
			account::account,
			external_actor::{ExternalActor, external_actor},
			message::{Message, message},
			shared::actor::{ActorId, ExternalActorReference},
		},
		features::external_authentication::external_authentication_request_schedule,
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

#[reducer]
/// Binds a third-party account to a internal account.
pub fn resolve_external_authentication_request(
	ctx: &ReducerContext, request_id: ExternalAuthenticationRequestId, is_approved: bool,
) -> Result<(), String> {
	let request = request_id.try_resolve(ctx)?;

	let ExternalAuthenticationRequest {
		requester, subject, ..
	} = &request;

	if is_approved {
		let mut account = requester.try_resolve(ctx)?;
		let external_actor = subject.try_resolve(ctx)?;

		ctx.db.external_actor().id().update(ExternalActor {
			account: Some(account.id),
			..external_actor
		});

		account.external_actors.push(subject.to_string());
		ctx.db.account().id().update(account);
	}

	ctx.db
		.external_authentication_request()
		.id()
		.delete(request_id);

	report_external_authentication_resolution(ctx, request, is_approved);

	Ok(())
}

#[reducer]
/// Unbinds a third-party account from a internal account.
pub fn unlink_external_actor(
	ctx: &ReducerContext, ext_actor_ref: ExternalActorReference,
) -> Result<(), String> {
	let mut account = ctx.sender.try_resolve(ctx)?;
	let external_actor = ext_actor_ref.try_resolve(ctx)?;

	if external_actor.account != Some(account.id) {
		return Err(format!(
			"Account {id} is not linked to the third-party account {ext_actor_ref}.",
			id = ctx.sender,
		));
	}

	ctx.db.external_actor().id().update(ExternalActor {
		account: None,
		..external_actor
	});

	account
		.external_actors
		.retain(|id| id != &ext_actor_ref.to_string());

	ctx.db.account().id().update(account);

	Ok(())
}

#[reducer]
/// Reports account link resolution outcome.
pub fn report_external_authentication_resolution(
	ctx: &ReducerContext, request: ExternalAuthenticationRequest, is_approved: bool,
) {
	let ExternalAuthenticationRequest {
		requester: _,
		subject,
		..
	} = request;

	let display_ext_ref = subject
		.parse::<ExternalActorReference>()
		.map_or(subject, |ext_ref| {
			format!(
				"{platform_name} account {fa_id}",
				platform_name = ext_ref.origin.to_string().capitalize(),
				fa_id = ext_ref.id,
			)
		});

	// TODO: Send DM instead, once DMs are implemented
	let result = ctx.db.message().try_insert(Message {
		id:        0,
		sender:    ctx.identity(),
		sent_at:   ctx.timestamp,
		author_id: ActorId::Internal(ctx.identity()),

		text: if is_approved {
			format!("{display_ext_ref} has been linked to your account.")
		} else {
			format!("Account link request for {display_ext_ref} has been rejected.")
		},
	});

	if let Err(err) = result {
		log::error!("Failed to send account link resolution message: {err}");
	}
}

#[reducer]
/// Removes an account link request.
/// Should only be invoked via a scheduled task.
pub fn scheduled_delete_external_authentication_request(
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
