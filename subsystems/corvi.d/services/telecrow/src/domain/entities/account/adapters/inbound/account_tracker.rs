use std::sync::Arc;

use capitalize::Capitalize;
use crowdcomm_sdk::{
	integrations::ports::{ExternalActorIdentification, ProfileImport},
	singularity::{
		ports::ProfileResolution,
		stdb::{
			DbConnection, ExternalActorReference, ExternalActorTableAccess,
			register_external_actor, update_external_actor_callsign, update_external_actor_profile,
		},
	},
};
use teloxide::types::User;

pub fn handle_telegram_user_update(
	ctx: Arc<DbConnection>, user_data: User, on_success: fn(msg: String), on_error: fn(msg: String),
) {
	let User {
		username: tg_username,
		..
	} = user_data.clone();

	let tg_ext_ref = user_data.into_actor_ref();
	let tg_profile_metadata = user_data.into_actor_profile_metadata();

	let ExternalActorReference {
		id: external_actor_external_id,
		origin,
	} = &tg_ext_ref;

	let platform_name = origin.to_string().capitalize();

	let external_actor = ctx.db.external_actor().id().find(&tg_ext_ref.to_string());

	if let Some(account) = external_actor {
		let profile = account.profile(&*ctx);

		if account.callsign != tg_username {
			let result = ctx
				.reducers
				.update_external_actor_callsign(tg_ext_ref.clone(), tg_username);

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
			let result = ctx
				.reducers
				.update_external_actor_profile(tg_ext_ref.clone(), Some(tg_profile_metadata));

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
		let result = ctx
			.reducers
			.register_external_actor(tg_ext_ref.clone(), tg_username, Some(tg_profile_metadata))
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
