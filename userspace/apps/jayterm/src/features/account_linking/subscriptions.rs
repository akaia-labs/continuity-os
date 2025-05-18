use corvutils::StringExtensions;
use crowdcomm_sdk::corvidx::{
	account_linking::AccountLinkRequestId,
	stdb::{
		DbConnection, TpAccountReference, ReducerEventContext, mirror_tp_profile,
		resolve_account_link_request, unlink_tp_account,
	},
};
use spacetimedb_sdk::Status;

pub fn subscribe(corvidx: &DbConnection) {
	corvidx
		.reducers
		.on_resolve_account_link_request(on_resolve_account_link_request);

	corvidx
		.reducers
		.on_unlink_tp_account(on_unlink_tp_account);

	corvidx
		.reducers
		.on_mirror_tp_profile(on_mirror_tp_profile);
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

fn on_unlink_tp_account(corvidx: &ReducerEventContext, reference: &TpAccountReference) {
	let TpAccountReference {
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

fn on_mirror_tp_profile(corvidx: &ReducerEventContext, reference: &TpAccountReference) {
	let TpAccountReference {
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
