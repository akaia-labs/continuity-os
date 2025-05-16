use corvutils::StringExtensions;
use crowdcomm_sdk::corvidx::stdb::{
	DbConnection, ForeignAccountReference, ReducerEventContext, mirror_foreign_profile,
	resolve_account_link_request, unlink_foreign_account,
};
use spacetimedb_sdk::Status;

pub fn subscribe(corvidx: &DbConnection) {
	corvidx
		.reducers
		.on_resolve_account_link_request(on_resolve_account_link_request);

	corvidx
		.reducers
		.on_unlink_foreign_account(on_unlink_foreign_account);

	corvidx
		.reducers
		.on_mirror_foreign_profile(on_mirror_foreign_profile);
}

// TODO: Send service DM to the particular requester instead
fn on_resolve_account_link_request(
	corvidx: &ReducerEventContext, reference: &ForeignAccountReference,
) {
	let ForeignAccountReference {
		id: external_identifier,
		platform_tag,
	} = reference;

	match &corvidx.event.status {
		| Status::Committed => {
			let message = format!(
				r#"
					{platform_tag} account {external_identifier}
					has been successfully linked to your account.
				"#
			)
			.squash_whitespace()
			.padded();

			println!("{message}")
		},

		| Status::Failed(err) => {
			let message =
				format!("Unable to link {external_identifier} {platform_tag} account:\n{err}")
					.padded();

			eprintln!("{message}")
		},

		| _ => {},
	}
}

fn on_unlink_foreign_account(corvidx: &ReducerEventContext, reference: &ForeignAccountReference) {
	let ForeignAccountReference {
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

fn on_mirror_foreign_profile(corvidx: &ReducerEventContext, reference: &ForeignAccountReference) {
	let ForeignAccountReference {
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
