use crowdcomm_sdk::corvidx::{
	AccountProfileMetadata, DbConnection, ForeignAccountReference, ReducerEventContext,
	import_foreign_account, update_foreign_account_profile,
};
use spacetimedb_sdk::Status;

fn on_foreign_account_import(
	corvidx: &ReducerEventContext, reference: &ForeignAccountReference, callsign: &Option<String>,
	metadata: &Option<AccountProfileMetadata>,
) {
	if let Status::Failed(err) = &corvidx.event.status {
		eprintln!("\n\nFailed to import account for {:?}: {}", callsign, err);
		println!("\n{:?}", reference);
		println!("{:?}\n\n", metadata);
	}
}

fn on_foreign_account_update(
	corvidx: &ReducerEventContext, reference: &ForeignAccountReference,
	metadata: &Option<AccountProfileMetadata>,
) {
	if let Status::Failed(err) = &corvidx.event.status {
		eprintln!("\n\nFailed to update account for {reference}: {err}");
		println!("{:?}\n\n", metadata);
	}
}

pub fn subscribe(corvidx: &DbConnection) {
	corvidx
		.reducers
		.on_import_foreign_account(on_foreign_account_import);

	corvidx
		.reducers
		.on_update_foreign_account_profile(on_foreign_account_update);
}
