use corvutils::StringExtensions;
use crowdcomm_sdk::corvidx::{
	external_authentication::AccountLinkRequestId,
	stdb::{
		DbConnection, ExternalActorReference, ReducerEventContext, mirror_external_profile,
		resolve_account_link_request, unlink_external_actor,
	},
};
use spacetimedb_sdk::Status;

pub fn subscribe(corvidx: &DbConnection) {
	corvidx
		.reducers
		.on_resolve_account_link_request(on_resolve_account_link_request);

	corvidx
		.reducers
		.on_unlink_external_actor(on_unlink_external_actor);

	corvidx
		.reducers
		.on_mirror_external_profile(on_mirror_external_profile);
}

// TODO: Send service DM to the particular requester instead
fn on_resolve_account_link_request(
	corvidx: &ReducerEventContext, request_id: &AccountLinkRequestId, is_approved: &bool,
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

fn on_unlink_external_actor(corvidx: &ReducerEventContext, reference: &ExternalActorReference) {
	let ExternalActorReference {
		id: external_identifier,
		platform_tag,
	} = reference;

	match &corvidx.event.status {
		| Status::Committed => {
			let message = format!(
				r#"
					{platform_tag} account {external_identifier}
					has been successfully unlinked from your account.
				"#
			)
			.squash_whitespace()
			.padded();

			println!("{message}")
		},

		| Status::Failed(err) => {
			let message =
				format!("Unable to unlink {external_identifier} {platform_tag} account:\n{err}")
					.padded();

			eprintln!("{message}")
		},

		| _ => {},
	}
}

fn on_mirror_external_profile(corvidx: &ReducerEventContext, reference: &ExternalActorReference) {
	let ExternalActorReference {
		id: external_identifier,
		platform_tag,
	} = reference;

	match &corvidx.event.status {
		| Status::Committed => {
			let message = format!(
				r#"
					Your profile has been updated to match the appearance of
					{external_identifier} {platform_tag} account.
				"#
			)
			.squash_whitespace()
			.padded();

			println!("{message}")
		},

		| Status::Failed(err) => {
			let message =
				format!("Unable to mirror {external_identifier} {platform_tag} profile:\n{err}")
					.padded();

			eprintln!("{message}")
		},

		| _ => {},
	}
}
