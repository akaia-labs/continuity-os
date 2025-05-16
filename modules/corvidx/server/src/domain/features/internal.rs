use spacetimedb::ReducerContext;

use crate::entities::native_account::{NativeAccountLocalRole, native_account};

// TODO: Return Result instead and move into a trait!
/// Asserts that the executor is an admin.
pub fn assert_admin(ctx: &ReducerContext) {
	assert!(
		ctx.db
			.native_account()
			.id()
			.find(ctx.sender)
			.map_or(false, |a| a.role == NativeAccountLocalRole::Admin),
		"{} is not an admin",
		ctx.sender,
	);
}
