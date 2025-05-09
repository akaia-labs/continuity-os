use spacetimedb::{ReducerContext, reducer};

use crate::{
	entities::local_account::{LocalAccount, LocalAccountId, LocalAccountRole, local_account},
	features::internal::assert_admin,
};

// ! HEADS UP! Don't forget to call `assert_admin(ctx);`
// ! in the first line of every admin reducer!

#[reducer]
/// Sets role for the specified account.
pub fn admin_set_account_role(
	ctx: &ReducerContext, account_id: LocalAccountId, role: LocalAccountRole,
) -> Result<(), String> {
	// TODO: Return an error instead of panicking and consider converting
	// TODO: this into `is_admin` on `LocalAccount` via permission control trait
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
