use spacetimedb::ReducerContext;

use crate::domain::entities::account::{AccountRole, account};

// TODO: Return Result instead and move into a trait!
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
