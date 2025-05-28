use spacetimedb::{ReducerContext, Table, reducer};

use super::{ExternalActor, ExternalActorReference, external_actor};
use crate::domain::entities::shared::actor::{ActorProfile, ActorProfileMetadata, actor_profile};

#[reducer]
/// Registers a local representation of the given 3rd party platform actor.
pub fn register_external_actor(
	ctx: &ReducerContext, ext_actor_ref: ExternalActorReference, callsign: Option<String>,
	metadata: Option<ActorProfileMetadata>,
) -> Result<(), String> {
	if ctx
		.db
		.external_actor()
		.id()
		.find(ext_actor_ref.to_string())
		.is_some()
	{
		return Err(format!(
			"External actor {ext_actor_ref} is already registered in the system.",
		));
	}

	ctx.db.external_actor().insert(ExternalActor {
		id: ext_actor_ref.to_string(),
		callsign,
		account: None,

		profile: Some(
			ctx.db
				.actor_profile()
				.insert(ActorProfile {
					id:       0,
					metadata: metadata.unwrap_or_default(),
				})
				.id,
		),
	});

	Ok(())
}

#[reducer]
/// Updates the local representation
/// of a 3rd party platform account handle / username.
pub fn update_external_actor_callsign(
	ctx: &ReducerContext, ext_actor_ref: ExternalActorReference, callsign: Option<String>,
) -> Result<(), String> {
	let account = ctx
		.db
		.external_actor()
		.id()
		.find(ext_actor_ref.to_string())
		.ok_or(format!(
			"External actor {ext_actor_ref} is not registered in the system."
		))?;

	ctx.db.external_actor().id().update(ExternalActor {
		callsign,
		..account
	});

	Ok(())
}

#[reducer]
/// Updates the local representation of a 3rd party platform actor profile.
pub fn update_external_actor_profile(
	ctx: &ReducerContext, ext_actor_ref: ExternalActorReference,
	metadata: Option<ActorProfileMetadata>,
) -> Result<(), String> {
	let account = ctx
		.db
		.external_actor()
		.id()
		.find(ext_actor_ref.to_string())
		.ok_or(format!(
			"External actor {ext_actor_ref} is not registered in the system."
		))?;

	let profile = if let Some(profile_id) = account.profile {
		ctx.db.actor_profile().id().update(ActorProfile {
			id:       profile_id,
			metadata: metadata.unwrap_or_default(),
		})
	} else {
		ctx.db.actor_profile().insert(ActorProfile {
			id:       0,
			metadata: metadata.unwrap_or_default(),
		})
	};

	ctx.db.external_actor().id().update(ExternalActor {
		profile: Some(profile.id),
		..account
	});

	Ok(())
}
