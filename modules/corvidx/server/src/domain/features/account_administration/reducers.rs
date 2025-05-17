use spacetimedb::{ReducerContext, reducer};

use crate::domain::{
	entities::native_account::{
		NativeAccount, NativeAccountId, NativeAccountLocalRole, native_account,
	},
	features::internal::assert_admin,
};

// ! HEADS UP! Don't forget to call `assert_admin(ctx);`
// ! in the first line of every admin reducer!

#[reducer]
/// Sets role for the specified account.
pub fn admin_set_account_role(
	ctx: &ReducerContext, account_id: NativeAccountId, role: NativeAccountLocalRole,
) -> Result<(), String> {
	// TODO: Return an error instead of panicking and consider converting
	// TODO: this into `is_admin` on `NativeAccount` via permission control trait
	assert_admin(ctx);

	if let Some(account) = ctx.db.native_account().id().find(account_id) {
		ctx.db.native_account().id().update(NativeAccount {
			role,
			updated_at: ctx.timestamp,
			..account
		});
	} else {
		return Err(format!("Account {} not found in the system", account_id));
	}

	Ok(())
}
