use spacetimedb::{ReducerContext, reducer};

use super::{super::validation::validate_callsign, Account, account};

#[reducer]
/// Accounts invoke this reducer to set their callsigns.
pub fn set_account_callsign(ctx: &ReducerContext, callsign: String) -> Result<(), String> {
	let account = ctx.db.account().id().find(ctx.sender).ok_or(format!(
		"Identity {id} does not have an account.",
		id = ctx.sender
	))?;

	let callsign = validate_callsign(callsign)?;

	ctx.db.account().id().update(Account {
		callsign,
		updated_at: ctx.timestamp,
		..account
	});

	Ok(())
}
