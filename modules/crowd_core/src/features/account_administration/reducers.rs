use spacetimedb::{ReducerContext, reducer};

use crate::{
	entities::{
		foreign_account::{ForeignAccount, ForeignAccountId, foreign_account},
		local_account::{LocalAccount, LocalAccountId, LocalAccountRole, local_account},
	},
	features::internal::assert_admin,
};

// ! HEADS UP! Don't forget to call `assert_admin(ctx);`
// ! in the first line of every admin reducer!

#[reducer]
/// Sets role for the specified account.
pub fn admin_set_account_role(
	ctx: &ReducerContext, account_id: LocalAccountId, role: LocalAccountRole,
) -> Result<(), String> {
	assert_admin(ctx);

	if let Some(account) = ctx.db.local_account().id().find(account_id) {
		ctx.db.local_account().id().update(LocalAccount {
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
pub fn admin_link_foreign_account(
	ctx: &ReducerContext, account_id: LocalAccountId, ext_account_id: ForeignAccountId,
) -> Result<(), String> {
	assert_admin(ctx);

	if let Some(ext_account) = ctx.db.foreign_account().id().find(ext_account_id.clone()) {
		if let Some(account) = ctx.db.local_account().id().find(account_id) {
			ctx.db.foreign_account().id().update(ForeignAccount {
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
