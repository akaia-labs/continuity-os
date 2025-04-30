use super::{tables::*, validators::*};
use spacetimedb::{ReducerContext, reducer};

#[reducer]
/// Clients invoke this reducer to set their account names.
pub fn set_callsign(ctx: &ReducerContext, callsign: String) -> Result<(), String> {
	let callsign = validate_callsign(callsign)?;

	if let Some(account) = ctx.db.account().identity().find(ctx.sender) {
		ctx.db.account().identity().update(Account {
			callsign: Some(callsign),
			updated_at: ctx.timestamp,
			..account
		});

		Ok(())
	} else {
		Err("Cannot set callsign for unknown account".to_string())
	}
}
