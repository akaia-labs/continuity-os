use spacetimedb::{ReducerContext, reducer};

use super::report::report_external_authentication_resolution;
use crate::{
	common::ports::RecordResolver,
	domain::entities::{
		account::account,
		action_request::{
			ExternalAuthenticationRequest, ExternalAuthenticationRequestId,
			external_authentication_request,
		},
		external_actor::{ExternalActor, external_actor},
	},
};

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
