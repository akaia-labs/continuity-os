use spacetimedb::{ReducerContext, Table, reducer};

use super::{ForeignAccount, ForeignAccountReference, foreign_account};
use crate::entities::account_profile::{
	AccountProfile, AccountProfileMetadata, AccountProfileOwnerId, account_profile,
};

#[reducer]
/// Registers an external account in the database.
pub fn import_foreign_account(
	ctx: &ReducerContext, reference: ForeignAccountReference, callsign: Option<String>,
	metadata: Option<AccountProfileMetadata>,
) -> Result<(), String> {
	let account = ctx.db.foreign_account().insert(ForeignAccount {
		id: reference.to_string(),
		callsign,
		owner_id: None,
		profile_id: None,
	});

	let profile = ctx.db.account_profile().insert(AccountProfile {
		id:       0,
		owner_id: AccountProfileOwnerId::ForeignAccountId(account.id.clone()),
		metadata: metadata.unwrap_or_default(),
	});

	ctx.db.foreign_account().id().update(ForeignAccount {
		profile_id: Some(profile.id),
		..account
	});

	Ok(())
}
