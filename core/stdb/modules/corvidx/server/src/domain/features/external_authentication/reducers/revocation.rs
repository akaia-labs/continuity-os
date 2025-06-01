use spacetimedb::{ReducerContext, reducer};

use crate::{
	common::ports::RecordResolver,
	domain::entities::{
		account::account,
		external_actor::{ExternalActor, ExternalActorReference, external_actor},
	},
};

#[reducer]
/// Unbinds a third-party identity from a internal account.
pub fn revoke_external_authentication(
	ctx: &ReducerContext, ext_actor_ref: ExternalActorReference,
) -> Result<(), String> {
	let mut account = ctx.sender.try_resolve(ctx)?;
	let external_actor = ext_actor_ref.try_resolve(ctx)?;

	if external_actor.account != Some(account.id) {
		return Err(format!(
			"Account {id} is not linked to the third-party account {ext_actor_ref}.",
			id = ctx.sender,
		));
	}

	ctx.db.external_actor().id().update(ExternalActor {
		account: None,
		..external_actor
	});

	account
		.external_actors
		.retain(|id| id != &ext_actor_ref.to_string());

	ctx.db.account().id().update(account);

	Ok(())
}
