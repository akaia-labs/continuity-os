use std::sync::Arc;

use capitalize::Capitalize;
use crowdcomm_sdk::{
	corvidx::{
		ports::ProfileResolution,
		stdb::{
			DbConnection, ExternalActorReference, ExternalActorTableAccess,
			register_external_actor, update_external_actor_callsign, update_external_actor_profile,
		},
	},
	integrations::ports::{ExternalActorIdentification, ProfileImport},
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

	let tg_exref = user_data.into_actor_ref();
	let tg_profile_metadata = user_data.into_actor_profile_metadata();

	let ExternalActorReference {
		id: external_actor_external_id,
		origin,
	} = &tg_exref;

	let platform_name = origin.to_string().capitalize();

	let external_actor = corvidx.db.external_actor().id().find(&tg_exref.to_string());

	if let Some(account) = external_actor {
		let profile = account.profile(&*corvidx);

		if account.callsign != tg_username {
			let result = corvidx
				.reducers
				.update_external_actor_callsign(tg_exref.clone(), tg_username);

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
				.update_external_actor_profile(tg_exref.clone(), Some(tg_profile_metadata));

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
			.register_external_actor(tg_exref.clone(), tg_username, Some(tg_profile_metadata))
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
