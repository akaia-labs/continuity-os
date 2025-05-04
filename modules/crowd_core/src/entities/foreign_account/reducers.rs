use spacetimedb::{ReducerContext, Table, reducer};

use super::{ForeignAccount, ForeignAccountReference, foreign_account};

#[reducer]
/// Registers an external account in the database.
pub fn add_foreign_account(
	ctx: &ReducerContext, reference: ForeignAccountReference,
) -> Result<(), String> {
	ctx.db.foreign_account().insert(ForeignAccount {
		id:         reference.to_string(),
		owner_id:   None,
		profile_id: None,
	});

	Ok(())
}
