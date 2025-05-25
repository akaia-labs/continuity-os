use std::sync::Arc;

use capitalize::Capitalize;
use crowdcomm_sdk::{
	corvidx::{
		ports::ProfileResolution,
		stdb::{
			DbConnection, ExternalActorReference, ExternalActorTableAccess, import_external_actor,
			update_external_actor_callsign, update_external_actor_profile,
		},
	},
	integrations::ports::{ExternalActorImport, ProfileImport},
};
use teloxide::types::User;

pub fn handle_telegram_user_update(
	corvidx: Arc<DbConnection>, user_data: User, on_success: fn(msg: String),
	on_error: fn(msg: String),
) {
	let User {
		username: tg_username,
		..
	} = user_data.clone();

	let tg_account_reference = user_data.into_account_reference();
	let tg_profile_metadata = user_data.into_profile_metadata();

	let ExternalActorReference {
		id: external_actor_external_id,
		platform_tag,
	} = &tg_account_reference;

	let platform_name = platform_tag.to_string().capitalize();

	let external_actor = corvidx
		.db
		.external_actor()
		.id()
		.find(&tg_account_reference.to_string());

	if let Some(account) = external_actor {
		let profile = account.profile(&*corvidx);

		if account.callsign != tg_username {
			let result = corvidx
				.reducers
				.update_external_actor_callsign(tg_account_reference.clone(), tg_username);

			match result {
				| Ok(_) => {
					on_success(format!(
						r#"
							Username change for {platform_name} account
							{external_actor_external_id} has been
							successfully reflected on its callsign.
						"#
					));
				},

				| Err(err) => {
					on_error(format!(
						r#"
							Unable to register username change for {platform_name}
							account {external_actor_external_id}: {err}
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
				.update_external_actor_profile(tg_account_reference.clone(), Some(tg_profile_metadata));

			match result {
				| Ok(_) => {
					on_success(format!(
						r#"
							{platform_name} profile record for account
							{external_actor_external_id} has been successfully updated.
						"#
					));
				},

				| Err(err) => {
					on_error(format!(
						r#"
							Unable to register profile change for {platform_name}
							account {external_actor_external_id}: {err}
						"#
					));
				},
			}
		};
	} else {
		let result = corvidx
			.reducers
			.import_external_actor(
				tg_account_reference.clone(),
				tg_username,
				Some(tg_profile_metadata),
			)
			.map_err(|e| e.to_string());

		match result {
			| Ok(_) => {
				on_success(format!(
					r#"
						{platform_name} account {external_actor_external_id}
						has been successfully imported.
					"#
				));
			},

			| Err(err) => {
				on_error(format!(
					r#"
						Unable to import {platform_name} account
						{external_actor_external_id}: {err}
					"#
				));
			},
		}
	}
}
