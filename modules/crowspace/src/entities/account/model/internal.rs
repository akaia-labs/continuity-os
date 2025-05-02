use spacetimedb::ReducerContext;

use super::{AccountRole, account};

/// Asserts that the executor is an admin.
pub fn assert_admin(ctx: &ReducerContext) {
	assert!(
		ctx.db
			.account()
			.id()
			.find(ctx.sender)
			.map_or(false, |a| a.role == AccountRole::Admin),
		"{} is not an admin",
		ctx.sender,
	);
}
