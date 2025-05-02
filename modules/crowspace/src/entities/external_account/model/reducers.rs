use spacetimedb::{ReducerContext, Table, reducer};

use crate::entities::external_platform::ExternalPlatformName;

use super::{ExternalAccount, ExternalAccountExternalId, external_account};

#[reducer]
/// Registers an external account in the database.
pub fn add_external_account(
	ctx: &ReducerContext, external_id: ExternalAccountExternalId,
	platform_name: ExternalPlatformName,
) -> Result<(), String> {
	ctx.db.external_account().insert(ExternalAccount {
		id: 0,
		external_id,
		platform_name,
		owner: None,
	});

	Ok(())
}
