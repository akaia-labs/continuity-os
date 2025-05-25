use spacetimedb::{ReducerContext, reducer};

use crate::{
	common::ports::RecordResolution,
	domain::entities::{
		actor_profile::{ActorProfile, account_profile},
		external_actor::ExternalActorReference,
	},
};

#[reducer]
/// Copies the linked third-party account's profile data
/// over to the internal account profile.
pub fn mirror_external_profile(
	ctx: &ReducerContext, reference: ExternalActorReference,
) -> Result<(), String> {
	let account = ctx.sender.try_resolve(ctx)?;
	let external_actor = reference.try_resolve(ctx)?;

	if external_actor.owner_id != Some(account.id) {
		return Err(format!(
			"Account {id} is not linked to the third-party account {reference}.",
			id = ctx.sender,
		));
	}

	let external_profile = if let Some(external_profile_id) = external_actor.profile {
		ctx.db.account_profile().id().find(external_profile_id)
	} else {
		None
	}
	.ok_or(format!(
		"External account {reference} does not have a profile."
	))?;

	ctx.db.account_profile().id().update(ActorProfile {
		id:       account.profile,
		metadata: external_profile.metadata,
	});

	Ok(())
}
