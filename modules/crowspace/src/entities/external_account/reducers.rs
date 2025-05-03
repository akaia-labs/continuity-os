use spacetimedb::{ReducerContext, Table, reducer};

use super::{ExternalAccount, ExternalAccountReference, external_account};

#[reducer]
/// Registers an external account in the database.
pub fn add_external_account(
	ctx: &ReducerContext, reference: ExternalAccountReference,
) -> Result<(), String> {
	ctx.db.external_account().insert(ExternalAccount {
		id:         reference.to_string(),
		owner_id:   None,
		profile_id: None,
	});

	Ok(())
}
