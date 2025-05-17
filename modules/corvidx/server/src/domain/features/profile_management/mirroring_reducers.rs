use spacetimedb::{ReducerContext, reducer};

use crate::{
	common::ports::RecordResolution,
	entities::{
		account_profile::{AccountProfile, account_profile},
		foreign_account::ForeignAccountReference,
	},
};

#[reducer]
/// Copies the linked foreign account's profile data
/// over to the native account profile.
pub fn mirror_foreign_profile(
	ctx: &ReducerContext, reference: ForeignAccountReference,
) -> Result<(), String> {
	let native_account = ctx.sender.try_resolve(ctx)?;
	let foreign_account = reference.try_resolve(ctx)?;

	if foreign_account.owner_id != native_account.id {
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
