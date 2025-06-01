use corvutils::StringExtensions;
use crowdcomm_sdk::corvidx::{
	external_authentication::ExternalAuthenticationRequestId,
	stdb::{
		DbConnection, ExternalActorReference, ReducerEventContext, mirror_external_profile,
		resolve_external_authentication_request, revoke_external_authentication,
	},
};
use spacetimedb_sdk::Status;

pub fn subscribe(corvidx: &DbConnection) {
	corvidx
		.reducers
		.on_resolve_external_authentication_request(on_resolve_external_authentication_request);

	corvidx
		.reducers
		.on_revoke_external_authentication(on_revoke_external_authentication);

	corvidx
		.reducers
		.on_mirror_external_profile(on_mirror_external_profile);
}

// TODO: Send service DM to the particular requester instead
fn on_resolve_external_authentication_request(
	corvidx: &ReducerEventContext, request_id: &ExternalAuthenticationRequestId, is_approved: &bool,
) {
	match &corvidx.event.status {
		| Status::Committed => {
			let message = format!(
				"Account link request {request_id} has been {outcome}.",
				outcome = if *is_approved { "approved" } else { "rejected" }
			)
			.padded();

			println!("{message}")
		},

		| Status::Failed(err) => {
			let message =
				format!("Unable to resolve account link request {request_id}:\n{err}").padded();

			eprintln!("{message}")
		},

		| _ => {},
	}
}

fn on_revoke_external_authentication(
	corvidx: &ReducerEventContext, reference: &ExternalActorReference,
) {
	let ExternalActorReference {
		id: external_identifier,
		origin,
	} = reference;

	match &corvidx.event.status {
		| Status::Committed => {
			let message = format!(
				r#"
					{origin} account {external_identifier}
					has been successfully unlinked from your account.
				"#
			)
			.squash_whitespace()
			.padded();

			println!("{message}")
		},

		| Status::Failed(err) => {
			let message =
				format!("Unable to unlink {external_identifier} {origin} account:\n{err}").padded();

			eprintln!("{message}")
		},

		| _ => {},
	}
}

fn on_mirror_external_profile(corvidx: &ReducerEventContext, reference: &ExternalActorReference) {
	let ExternalActorReference {
		id: external_identifier,
		origin,
	} = reference;

	match &corvidx.event.status {
		| Status::Committed => {
			let message = format!(
				r#"
					Your profile has been updated to match the appearance of
					{external_identifier} {origin} account.
				"#
			)
			.squash_whitespace()
			.padded();

			println!("{message}")
		},

		| Status::Failed(err) => {
			let message =
				format!("Unable to mirror {external_identifier} {origin} profile:\n{err}").padded();

			eprintln!("{message}")
		},

		| _ => {},
	}
}
