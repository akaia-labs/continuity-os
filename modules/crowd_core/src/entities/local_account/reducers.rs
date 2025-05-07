use spacetimedb::{ReducerContext, reducer};

use super::{tables::*, validation::*};
use crate::entities::{
	account_profile::{AccountProfile, account_profile},
	foreign_account::{ForeignAccount, ForeignAccountReference, foreign_account},
};

#[reducer]
/// Clients invoke this reducer to set their callsigns.
pub fn set_account_callsign(ctx: &ReducerContext, callsign: String) -> Result<(), String> {
	let account = ctx.db.local_account().id().find(ctx.sender).ok_or(format!(
		"Identity {id} does not have an account.",
		id = ctx.sender
	))?;

	let callsign = validate_callsign(callsign)?;

	ctx.db.local_account().id().update(LocalAccount {
		callsign,
		updated_at: ctx.timestamp,
		..account
	});

	Ok(())
}

#[reducer]
/// Binds a foreign account to a local account.
pub fn link_foreign_account(
	ctx: &ReducerContext, reference: ForeignAccountReference,
) -> Result<(), String> {
	let local_account = ctx.db.local_account().id().find(ctx.sender).ok_or(format!(
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

	ctx.db.foreign_account().id().update(ForeignAccount {
		owner_id: Some(local_account.id),
		..foreign_account
	});

	Ok(())
}

#[reducer]
/// Unbinds a foreign account from a local account.
pub fn unlink_foreign_account(
	ctx: &ReducerContext, reference: ForeignAccountReference,
) -> Result<(), String> {
	let local_account = ctx.db.local_account().id().find(ctx.sender).ok_or(format!(
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

	if foreign_account.owner_id.is_some() && foreign_account.owner_id.unwrap() != local_account.id {
		return Err(format!(
			"Account {id} is not linked to the foreign account {reference}.",
			id = ctx.sender,
		));
	}

	ctx.db.foreign_account().id().update(ForeignAccount {
		owner_id: None,
		..foreign_account
	});

	Ok(())
}

#[reducer]
/// Copies the linked foreign account's profile data
/// over to the local account profile.
pub fn mirror_foreign_profile(
	ctx: &ReducerContext, reference: ForeignAccountReference,
) -> Result<(), String> {
	let local_account = ctx.db.local_account().id().find(ctx.sender).ok_or(format!(
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

	if foreign_account.owner_id.is_some() && foreign_account.owner_id.unwrap() != local_account.id {
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
		id:       local_account.profile_id,
		metadata: foreign_profile.metadata,
	});

	Ok(())
}
