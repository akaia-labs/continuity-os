use crowdcomm_sdk::corvidx::stdb::{
	AccountProfileMetadata, DbConnection, TpAccountReference, ReducerEventContext,
	import_tp_account, update_tp_account_profile,
};
use spacetimedb_sdk::Status;

fn on_tp_account_import(
	corvidx: &ReducerEventContext, reference: &TpAccountReference, callsign: &Option<String>,
	metadata: &Option<AccountProfileMetadata>,
) {
	if let Status::Failed(err) = &corvidx.event.status {
		eprintln!("\n\nFailed to import account for {:?}: {}", callsign, err);
		println!("\n{:?}", reference);
		println!("{:?}\n\n", metadata);
	}
}

fn on_tp_account_update(
	corvidx: &ReducerEventContext, reference: &TpAccountReference,
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
		.on_import_tp_account(on_tp_account_import);

	corvidx
		.reducers
		.on_update_tp_account_profile(on_tp_account_update);
}
