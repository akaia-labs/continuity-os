use spacetimedb::{ReducerContext, reducer};

use super::{tables::*, validation::*};
use crate::entities::external_account::{ExternalAccount, ExternalAccountId, external_account};

#[reducer]
/// Clients invoke this reducer to set their account names.
pub fn set_callsign(ctx: &ReducerContext, callsign: String) -> Result<(), String> {
	let callsign = validate_callsign(callsign)?;

	if let Some(account) = ctx.db.account().id().find(ctx.sender) {
		ctx.db.account().id().update(Account {
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
pub fn link_external_account(
	ctx: &ReducerContext, ext_account_id: ExternalAccountId,
) -> Result<(), String> {
	if let Some(ext_account) = ctx.db.external_account().id().find(ext_account_id.clone()) {
		if let Some(account) = ctx.db.account().id().find(ctx.sender) {
			ctx.db.external_account().id().update(ExternalAccount {
				owner: Some(account.id),
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
pub fn unlink_external_account(
	ctx: &ReducerContext, ext_account_id: ExternalAccountId,
) -> Result<(), String> {
	if let Some(ext_account) = ctx.db.external_account().id().find(ext_account_id) {
		if let Some(_) = ctx.db.account().id().find(ctx.sender) {
			ctx.db.external_account().id().update(ExternalAccount {
				owner: None,
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
