use super::{tables::*, validators::*};
use spacetimedb::{ReducerContext, reducer};

#[reducer]
/// Clients invoke this reducer to set their user names.
pub fn set_name(ctx: &ReducerContext, name: String) -> Result<(), String> {
	let name = validate_name(name)?;

	if let Some(user) = ctx.db.user().identity().find(ctx.sender) {
		ctx.db.user().identity().update(User {
			name: Some(name),
			..user
		});

		Ok(())
	} else {
		Err("Cannot set name for unknown user".to_string())
	}
}
