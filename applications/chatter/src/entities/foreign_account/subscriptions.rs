use crowcomm::crowd_core::{
	AccountProfileMetadata, DbConnection, ForeignAccountReference, ReducerEventContext,
	import_foreign_account, update_foreign_account,
};
use spacetimedb_sdk::Status;

fn on_foreign_account_import(
	ctx: &ReducerEventContext, reference: &ForeignAccountReference, callsign: &Option<String>,
	metadata: &Option<AccountProfileMetadata>,
) {
	if let Status::Failed(err) = &ctx.event.status {
		eprintln!("\n\nFailed to import account for {:?}: {}", callsign, err);
		println!("\n{:?}", reference);
		println!("{:?}\n\n", metadata);
	}
}

fn on_foreign_account_update(
	ctx: &ReducerEventContext, reference: &ForeignAccountReference, callsign: &Option<String>,
	metadata: &Option<AccountProfileMetadata>,
) {
	if let Status::Failed(err) = &ctx.event.status {
		eprintln!("\n\nFailed to update account for {:?}: {}", callsign, err);
		println!("\n{:?}", reference);
		println!("{:?}\n\n", metadata);
	}
}

pub fn subscribe(core_ctx: &DbConnection) {
	core_ctx
		.reducers
		.on_import_foreign_account(on_foreign_account_import);

	core_ctx
		.reducers
		.on_update_foreign_account(on_foreign_account_update);
}
