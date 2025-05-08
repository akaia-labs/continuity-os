use spacetimedb::{ReducerContext, reducer};

use crate::entities::{
	foreign_account::{ForeignAccount, ForeignAccountReference, foreign_account},
	local_account::local_account,
};

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
