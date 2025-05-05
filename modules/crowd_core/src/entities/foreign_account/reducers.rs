use spacetimedb::{ReducerContext, Table, reducer};

use super::{ForeignAccount, ForeignAccountReference, foreign_account};
use crate::entities::account_profile::{AccountProfile, AccountProfileMetadata, account_profile};

#[reducer]
/// Registers a representation of a 3rd party platform account in the database.
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
		metadata: metadata.unwrap_or_default(),
	});

	ctx.db.foreign_account().id().update(ForeignAccount {
		profile_id: Some(profile.id),
		..account
	});

	Ok(())
}

#[reducer]
/// Updates the representation of a 3rd party platform account in the database.
pub fn update_foreign_account(
	ctx: &ReducerContext, reference: ForeignAccountReference, callsign: Option<String>,
	metadata: Option<AccountProfileMetadata>,
) -> Result<(), String> {
	let account = ctx
		.db
		.foreign_account()
		.id()
		.find(reference.to_string())
		.ok_or(format!(
			"Foreign account {reference} is not registered in the system."
		))?;

	let profile = if let Some(profile_id) = account.profile_id {
		ctx.db.account_profile().id().update(AccountProfile {
			id:       profile_id,
			metadata: metadata.unwrap_or_default(),
		})
	} else {
		ctx.db.account_profile().insert(AccountProfile {
			id:       0,
			metadata: metadata.unwrap_or_default(),
		})
	};

	let account_update = if callsign != account.callsign {
		ForeignAccount {
			callsign,
			profile_id: Some(profile.id),
			..account
		}
	} else {
		ForeignAccount {
			profile_id: Some(profile.id),
			..account
		}
	};

	ctx.db.foreign_account().id().update(account_update);

	Ok(())
}
