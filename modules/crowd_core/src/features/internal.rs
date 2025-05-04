use spacetimedb::ReducerContext;

use crate::entities::local_account::{LocalAccountRole, local_account};

/// Asserts that the executor is an admin.
pub fn assert_admin(ctx: &ReducerContext) {
	assert!(
		ctx.db
			.local_account()
			.id()
			.find(ctx.sender)
			.map_or(false, |a| a.role == LocalAccountRole::Admin),
		"{} is not an admin",
		ctx.sender,
	);
}
