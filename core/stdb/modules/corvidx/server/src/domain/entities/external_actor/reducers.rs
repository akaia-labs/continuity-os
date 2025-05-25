use spacetimedb::{ReducerContext, Table, reducer};

use super::{ExternalActor, ExternalActorReference, external_actor};
use crate::domain::entities::actor_profile::{ActorProfile, ActorProfileMetadata, account_profile};

#[reducer]
/// Registers a local representation of the given 3rd party platform account.
pub fn import_external_actor(
	ctx: &ReducerContext, reference: ExternalActorReference, callsign: Option<String>,
	metadata: Option<ActorProfileMetadata>,
) -> Result<(), String> {
	if ctx
		.db
		.external_actor()
		.id()
		.find(reference.to_string())
		.is_some()
	{
		return Err(format!(
			"External account {reference} is already registered in the system.",
		));
	}

	ctx.db.external_actor().insert(ExternalActor {
		id: reference.to_string(),
		callsign,
		owner_id: None,

		profile: Some(
			ctx.db
				.account_profile()
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
	ctx: &ReducerContext, reference: ExternalActorReference, callsign: Option<String>,
) -> Result<(), String> {
	let account = ctx
		.db
		.external_actor()
		.id()
		.find(reference.to_string())
		.ok_or(format!(
			"External account {reference} is not registered in the system."
		))?;

	ctx.db.external_actor().id().update(ExternalActor {
		callsign,
		..account
	});

	Ok(())
}

#[reducer]
/// Updates the local representation of a 3rd party platform account profile.
pub fn update_external_actor_profile(
	ctx: &ReducerContext, reference: ExternalActorReference, metadata: Option<ActorProfileMetadata>,
) -> Result<(), String> {
	let account = ctx
		.db
		.external_actor()
		.id()
		.find(reference.to_string())
		.ok_or(format!(
			"External account {reference} is not registered in the system."
		))?;

	let profile = if let Some(profile_id) = account.profile {
		ctx.db.account_profile().id().update(ActorProfile {
			id:       profile_id,
			metadata: metadata.unwrap_or_default(),
		})
	} else {
		ctx.db.account_profile().insert(ActorProfile {
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
