use spacetimedb::{ReducerContext, reducer};

use crate::{
	common::ports::RecordResolver,
	domain::entities::{
		external_actor::ExternalActorReference,
		shared::actor::{ActorProfile, actor_profile},
	},
};

#[reducer]
/// Copies the linked third-party profile data
/// over to the internal account profile.
pub fn mirror_external_profile(
	ctx: &ReducerContext, ext_actor_ref: ExternalActorReference,
) -> Result<(), String> {
	let account = ctx.sender.try_resolve(ctx)?;
	let external_actor = ext_actor_ref.try_resolve(ctx)?;

	if external_actor.account != Some(account.id) {
		return Err(format!(
			"Account {id} is not linked to the third-party account {ext_actor_ref}.",
			id = ctx.sender,
		));
	}

	let external_profile = if let Some(external_profile_id) = external_actor.profile {
		ctx.db.actor_profile().id().find(external_profile_id)
	} else {
		None
	}
	.ok_or(format!(
		"External actor {ext_actor_ref} does not have a profile."
	))?;

	ctx.db.actor_profile().id().update(ActorProfile {
		id:       account.profile,
		metadata: external_profile.metadata,
	});

	Ok(())
}
