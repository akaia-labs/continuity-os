use std::sync::Arc;

use capitalize::Capitalize;
use crowdcomm_sdk::{
	corvidx::{
		ports::ProfileResolution,
		stdb::{
			DbConnection, TpAccountReference, TpAccountTableAccess, import_tp_account,
			update_tp_account_callsign, update_tp_account_profile,
		},
	},
	integrations::ports::{ProfileImport, TpAccountImport},
};
use teloxide::types::User;

pub fn on_user_update(
	corvidx: Arc<DbConnection>, user_data: User, on_success: fn(msg: String),
	on_error: fn(msg: String),
) {
	let User {
		username: tg_username,
		..
	} = user_data.clone();

	let tg_account_reference = user_data.into_account_reference();
	let tg_profile_metadata = user_data.into_profile_metadata();

	let TpAccountReference {
		id: tp_account_external_id,
		platform_tag,
	} = &tg_account_reference;

	let platform_name = platform_tag.to_string().capitalize();

	let tp_account = corvidx
		.db
		.tp_account()
		.id()
		.find(&tg_account_reference.to_string());

	if let Some(account) = tp_account {
		let profile = account.profile(&*corvidx);

		if account.callsign != tg_username {
			let result = corvidx
				.reducers
				.update_tp_account_callsign(tg_account_reference.clone(), tg_username);

			match result {
				| Ok(_) => {
					on_success(format!(
						r#"
							Username change for {platform_name} account
							{tp_account_external_id} has been
							successfully reflected on its callsign.
						"#
					));
				},

				| Err(err) => {
					on_error(format!(
						r#"
							Unable to register username change for {platform_name}
							account {tp_account_external_id}: {err}
						"#
					));
				},
			}
		}

		if profile.is_none()
			|| profile.is_some_and(|profile| profile.metadata != tg_profile_metadata)
		{
			let result = corvidx
				.reducers
				.update_tp_account_profile(tg_account_reference.clone(), Some(tg_profile_metadata));

			match result {
				| Ok(_) => {
					on_success(format!(
						r#"
							{platform_name} profile record for account
							{tp_account_external_id} has been successfully updated.
						"#
					));
				},

				| Err(err) => {
					on_error(format!(
						r#"
							Unable to register profile change for {platform_name}
							account {tp_account_external_id}: {err}
						"#
					));
				},
			}
		};
	} else {
		let result = corvidx
			.reducers
			.import_tp_account(
				tg_account_reference.clone(),
				tg_username,
				Some(tg_profile_metadata),
			)
			.map_err(|e| e.to_string());

		match result {
			| Ok(_) => {
				on_success(format!(
					r#"
						{platform_name} account {tp_account_external_id}
						has been successfully imported.
					"#
				));
			},

			| Err(err) => {
				on_error(format!(
					r#"
						Unable to import {platform_name} account
						{tp_account_external_id}: {err}
					"#
				));
			},
		}
	}
}
