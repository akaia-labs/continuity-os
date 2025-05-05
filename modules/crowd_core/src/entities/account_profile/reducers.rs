use spacetimedb::{ReducerContext, Table, TryInsertError, reducer};

use super::{AccountProfile, AccountProfileMetadata, AccountProfileOwnerId, account_profile};
use crate::entities::account_profile::account_profile__TableHandle;

type ProfileCreationError = TryInsertError<account_profile__TableHandle>;

#[reducer]
pub fn create_account_profile(
	ctx: &ReducerContext, owner_id: AccountProfileOwnerId, metadata: AccountProfileMetadata,
) -> Result<(), String> {
	ctx.db
		.account_profile()
		.try_insert(AccountProfile {
			id: 0,
			owner_id,
			metadata,
		})
		.map(|_| ())
		.map_err(|error: ProfileCreationError| {
			format!("Unable to create account profile: {}", error)
		})
}
