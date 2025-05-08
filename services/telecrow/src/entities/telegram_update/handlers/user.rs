use std::sync::Arc;

use crowdcomm::corvidx::{
	DbConnection, ForeignAccountTableAccess,
	account::ForeignAccountImport,
	import_foreign_account,
	profile::{ProfileImport, ProfileRetrieval},
	update_foreign_account,
};
use teloxide::types::User;

pub fn on_user_update(core_ctx: Arc<DbConnection>, user_data: User) {
	let username = user_data.clone().username;
	let account_reference = user_data.into_account_reference();
	let account_metadata = user_data.into_profile_metadata();

	if let Some(account) = core_ctx
		.db
		.foreign_account()
		.id()
		.find(&account_reference.to_string())
	{
		if account
			.profile(&*core_ctx)
			.is_some_and(|profile| profile.metadata != account_metadata)
		{
			let _result = core_ctx.reducers.update_foreign_account(
				account_reference,
				username,
				Some(account_metadata),
			);
		}
	} else {
		let _result = core_ctx.reducers.import_foreign_account(
			account_reference,
			username,
			Some(account_metadata),
		);
	}
}
