use spacetimedb::{ReducerContext, reducer};

use crate::{
	common::ports::RecordResolution,
	domain::entities::{
		account_profile::{AccountProfile, account_profile},
		tp_account::TpAccountReference,
	},
};

#[reducer]
/// Copies the linked third-party account's profile data
/// over to the native account profile.
pub fn mirror_tp_profile(
	ctx: &ReducerContext, reference: TpAccountReference,
) -> Result<(), String> {
	let native_account = ctx.sender.try_resolve(ctx)?;
	let tp_account = reference.try_resolve(ctx)?;

	if tp_account.owner_id != Some(native_account.id) {
		return Err(format!(
			"Account {id} is not linked to the third-party account {reference}.",
			id = ctx.sender,
		));
	}

	let tp_profile = if let Some(tp_profile_id) = tp_account.profile_id {
		ctx.db.account_profile().id().find(tp_profile_id)
	} else {
		None
	}
	.ok_or(format!("Tp account {reference} does not have a profile."))?;

	ctx.db.account_profile().id().update(AccountProfile {
		id:       native_account.profile_id,
		metadata: tp_profile.metadata,
	});

	Ok(())
}
