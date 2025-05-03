use spacetimedb::{ReducerContext, reducer};

use crate::{
	entities::{
		external_account::{ExternalAccount, ExternalAccountId, external_account},
		internal_account::{Account, AccountId, AccountRole, account},
	},
	features::internal::assert_admin,
};

// ! HEADS UP! Don't forget to call `assert_admin(ctx);`
// ! in the first line of every admin reducer!

#[reducer]
/// Sets role for the specified account.
pub fn admin_set_account_role(
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
/// Sets role for the specified account.
pub fn admin_link_external_account(
	ctx: &ReducerContext, account_id: AccountId, ext_account_id: ExternalAccountId,
) -> Result<(), String> {
	assert_admin(ctx);

	if let Some(ext_account) = ctx.db.external_account().id().find(ext_account_id.clone()) {
		if let Some(account) = ctx.db.account().id().find(account_id) {
			ctx.db.external_account().id().update(ExternalAccount {
				owner_id: Some(account.id),
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
