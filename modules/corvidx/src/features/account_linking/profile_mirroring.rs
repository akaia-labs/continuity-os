use spacetimedb::{ReducerContext, reducer};

use crate::entities::{
	account_profile::{AccountProfile, account_profile},
	foreign_account::{ForeignAccountReference, foreign_account},
	native_account::native_account,
};

#[reducer]
/// Copies the linked foreign account's profile data
/// over to the local account profile.
pub fn mirror_foreign_profile(
	ctx: &ReducerContext, reference: ForeignAccountReference,
) -> Result<(), String> {
	let native_account = ctx.db.native_account().id().find(ctx.sender).ok_or(format!(
		"Identity {id} does not have an account.",
		id = ctx.sender
	))?;

	let foreign_account = ctx
		.db
		.foreign_account()
		.id()
		.find(reference.to_string())
		.ok_or(format!(
			"Foreign account {reference} is not registered in the system."
		))?;

	if foreign_account.owner_id.is_some() && foreign_account.owner_id.unwrap() != native_account.id {
		return Err(format!(
			"Account {id} is not linked to the foreign account {reference}.",
			id = ctx.sender,
		));
	}

	let foreign_profile = if let Some(foreign_profile_id) = foreign_account.profile_id {
		ctx.db.account_profile().id().find(foreign_profile_id)
	} else {
		None
	}
	.ok_or(format!(
		"Foreign account {reference} does not have a profile."
	))?;

	ctx.db.account_profile().id().update(AccountProfile {
		id:       native_account.profile_id,
		metadata: foreign_profile.metadata,
	});

	Ok(())
}
