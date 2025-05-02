use crate::entities::external_account::{ExternalAccount, ExternalAccountId, external_account};

use super::{internal::assert_admin, tables::*, validators::*};
use spacetimedb::{ReducerContext, reducer};

#[reducer]
/// Clients invoke this reducer to set their account names.
pub fn set_callsign(ctx: &ReducerContext, callsign: String) -> Result<(), String> {
	let callsign = validate_callsign(callsign)?;

	if let Some(account) = ctx.db.account().id().find(ctx.sender) {
		ctx.db.account().id().update(Account {
			callsign: Some(callsign),
			updated_at: ctx.timestamp,
			..account
		});

		Ok(())
	} else {
		Err("Cannot set callsign for unknown account".to_string())
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
			return Err(format!("{} does not have an internal account", ctx.sender));
		};
	} else {
		return Err(format!(
			"External account {} not found in the system",
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
			return Err(format!("{} does not have an internal account", ctx.sender));
		}
	} else {
		return Err(format!(
			"External account {} not found in the system",
			ctx.sender
		));
	}

	Ok(())
}

/**
 ** Administration
 *
 *! HEADS UP! Don't forget to call `assert_admin(ctx);` in the first line of every admin reducer!
 */

#[reducer]
/// Sets role for the specified account if the sender is an admin.
pub fn set_account_role(
	ctx: &ReducerContext, account_id: AccountId, role: AccountRole,
) -> Result<(), String> {
	assert_admin(ctx);

	if let Some(account) = ctx.db.account().id().find(account_id) {
		ctx.db.account().id().update(Account {
			role,
			updated_at: ctx.timestamp,
			..account
		});
	} else {
		return Err(format!("Account {} not found in the system", account_id));
	}

	Ok(())
}

#[reducer]
/// Sets role for the specified account if the sender is an admin.
pub fn admin_link_external_account(
	ctx: &ReducerContext, account_id: AccountId, ext_account_id: ExternalAccountId,
) -> Result<(), String> {
	assert_admin(ctx);

	if let Some(ext_account) = ctx.db.external_account().id().find(ext_account_id.clone()) {
		if let Some(account) = ctx.db.account().id().find(account_id) {
			ctx.db.external_account().id().update(ExternalAccount {
				owner: Some(account.id),
				..ext_account
			});
		} else {
			return Err(format!("Account {} not found in the system", account_id));
		};
	} else {
		return Err(format!(
			"External account {} not found in the system",
			ext_account_id
		));
	}

	Ok(())
}
