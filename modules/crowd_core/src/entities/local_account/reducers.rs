use spacetimedb::{ReducerContext, reducer};

use super::{tables::*, validation::*};
use crate::entities::foreign_account::{ForeignAccount, ForeignAccountId, foreign_account};

#[reducer]
/// Clients invoke this reducer to set their account names.
pub fn set_callsign(ctx: &ReducerContext, callsign: String) -> Result<(), String> {
	let callsign = validate_callsign(callsign)?;

	if let Some(account) = ctx.db.local_account().id().find(ctx.sender) {
		ctx.db.local_account().id().update(LocalAccount {
			callsign,
			updated_at: ctx.timestamp,
			..account
		});

		Ok(())
	} else {
		Err(format!("{} does not have an internal account.", ctx.sender))
	}
}

#[reducer]
/// Binds an external account to an internal account
pub fn link_foreign_account(
	ctx: &ReducerContext, ext_account_id: ForeignAccountId,
) -> Result<(), String> {
	if let Some(ext_account) = ctx.db.foreign_account().id().find(ext_account_id.clone()) {
		if let Some(account) = ctx.db.local_account().id().find(ctx.sender) {
			ctx.db.foreign_account().id().update(ForeignAccount {
				owner_id: Some(account.id),
				..ext_account
			});
		} else {
			return Err(format!("{} does not have an internal account.", ctx.sender));
		};
	} else {
		return Err(format!(
			"External account {} not found in the system.",
			ext_account_id
		));
	}

	Ok(())
}

#[reducer]
/// Unbinds an external account from an internal account
pub fn unlink_foreign_account(
	ctx: &ReducerContext, ext_account_id: ForeignAccountId,
) -> Result<(), String> {
	if let Some(ext_account) = ctx.db.foreign_account().id().find(ext_account_id) {
		if let Some(_) = ctx.db.local_account().id().find(ctx.sender) {
			ctx.db.foreign_account().id().update(ForeignAccount {
				owner_id: None,
				..ext_account
			});
		} else {
			return Err(format!("{} does not have an internal account.", ctx.sender));
		}
	} else {
		return Err(format!(
			"External account {} not found in the system.",
			ctx.sender
		));
	}

	Ok(())
}
