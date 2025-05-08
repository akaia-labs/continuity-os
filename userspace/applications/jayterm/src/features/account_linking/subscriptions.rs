use crowdcomm::corvidx::{
	DbConnection, ForeignAccountReference, ReducerEventContext, link_foreign_account,
	mirror_foreign_profile, unlink_foreign_account,
};
use spacetimedb_sdk::Status;

pub fn subscribe(corvidx: &DbConnection) {
	corvidx
		.reducers
		.on_link_foreign_account(on_link_foreign_account);

	corvidx
		.reducers
		.on_unlink_foreign_account(on_unlink_foreign_account);

	corvidx
		.reducers
		.on_mirror_foreign_profile(on_mirror_foreign_profile);
}

fn on_link_foreign_account(corvidx: &ReducerEventContext, reference: &ForeignAccountReference) {
	match &corvidx.event.status {
		| Status::Committed => {
			print!("\nForeign account {reference} has been successfully linked to your account.\n")
		},

		| Status::Failed(err) => {
			eprintln!("\nUnable to link foreign account {reference}: {}\n", err)
		},

		| _ => {},
	}
}

fn on_unlink_foreign_account(corvidx: &ReducerEventContext, reference: &ForeignAccountReference) {
	let ForeignAccountReference {
		id: external_identifier,
		platform_name,
	} = reference;

	match &corvidx.event.status {
		| Status::Committed => {
			print!(
				"\n{platform_name} account {external_identifier} has been successfully unlinked \
				 from your account.\n"
			)
		},

		| Status::Failed(err) => {
			eprintln!("\nUnable to unlink {external_identifier} {platform_name} account: {err}\n",)
		},

		| _ => {},
	}
}

fn on_mirror_foreign_profile(corvidx: &ReducerEventContext, reference: &ForeignAccountReference) {
	let ForeignAccountReference {
		id: external_identifier,
		platform_name,
	} = reference;

	match &corvidx.event.status {
		| Status::Committed => {
			let message = format!(
				r#"
					Your profile has been updated to match the appearance of
					{external_identifier} {platform_name} account.
				"#
			)
			.split_whitespace()
			.collect::<Vec<_>>()
			.join(" ");

			print!("\n{message}\n")
		},

		| Status::Failed(err) => {
			eprintln!(
				"\nUnable to mirror {external_identifier} {platform_name} profile: {}\n",
				err
			)
		},

		| _ => {},
	}
}
