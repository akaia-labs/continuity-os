use std::sync::Arc;

use crowdcomm::corvidx::{
	DbConnection, ForeignAccountTableAccess,
	account::ForeignAccountImport,
	import_foreign_account,
	profile::{ProfileImport, ProfileRetrieval},
	update_foreign_account,
};
use teloxide::types::User;

pub fn on_user_update(corvidx: Arc<DbConnection>, user_data: User) {
	let username = user_data.clone().username;
	let tg_account_reference = user_data.into_account_reference();
	let tg_profile_metadata = user_data.into_profile_metadata();

	let foreign_account = corvidx
		.db
		.foreign_account()
		.id()
		.find(&tg_account_reference.to_string());

	if let Some(account) = foreign_account {
		let profile = account.profile(&*corvidx);

		if account.callsign != username
			|| profile.is_none()
			|| profile.is_some_and(|profile| profile.metadata != tg_profile_metadata)
		{
			let result = corvidx.reducers.update_foreign_account(
				tg_account_reference,
				username,
				Some(tg_profile_metadata),
			);
		};
	} else {
		let result = corvidx
			.reducers
			.import_foreign_account(
				tg_account_reference.clone(),
				username,
				Some(tg_profile_metadata),
			)
			.map_err(|e| e.to_string());

		match result {
			| Ok(_) => {
				println!("Account {tg_account_reference} has been successfully imported.")
			},

			| Err(e) => {
				eprintln!("Unable to import account {tg_account_reference}: {e}")
			},
		}
	}
}
