use std::{pin::Pin, sync::Arc};

use crowcomm::{
	crowd_core::{
		AccountProfileMetadata, DbConnection, ForeignAccountReference, ReducerEventContext,
		account::ForeignAccountImport, import_foreign_account, profile::ProfileImport,
	},
	telegram,
};
use spacetimedb_sdk::Status;
use teloxide::{Bot, RequestError, respond};

pub fn handle_updates(
	core_ctx: Arc<DbConnection>,
) -> impl Fn(telegram::User, Bot) -> Pin<Box<dyn Future<Output = Result<(), RequestError>> + Send>>
{
	move |tg_user_data: telegram::User, _bot: Bot| {
		let ctx = core_ctx.clone();

		let account_reference = tg_user_data.into_account_reference();

		// TODO: Check if the account is already registered and update its profile
		// TODO: instead if this is the case
		Box::pin(async move {
			let _result = ctx.reducers.import_foreign_account(
				account_reference,
				tg_user_data.clone().username,
				Some(tg_user_data.into_profile_metadata()),
			);

			respond(())
		})
	}
}

fn on_foreign_account_import(
	ctx: &ReducerEventContext, reference: &ForeignAccountReference, callsign: &Option<String>,
	metadata: &Option<AccountProfileMetadata>,
) {
	println!("{:?}", reference);
	println!("{:?}", metadata);

	if let Status::Failed(err) = &ctx.event.status {
		eprintln!("Failed to import account for {:?}: {}", callsign, err);
	}
}

pub fn subscribe(core_ctx: &DbConnection) {
	core_ctx
		.reducers
		.on_import_foreign_account(on_foreign_account_import);
}
