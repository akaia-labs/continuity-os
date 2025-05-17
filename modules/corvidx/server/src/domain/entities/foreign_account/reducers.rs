use spacetimedb::{ReducerContext, Table, reducer};

use super::{ForeignAccount, ForeignAccountReference, foreign_account};
use crate::{
	common::ports::RecordResolution,
	domain::entities::{
		account_profile::{AccountProfile, AccountProfileMetadata, account_profile},
		native_account::{NativeAccount, native_account},
	},
};

#[reducer]
/// Registers a local representation of the given 3rd party platform account.
pub fn import_foreign_account(
	ctx: &ReducerContext, reference: ForeignAccountReference, callsign: Option<String>,
	metadata: Option<AccountProfileMetadata>,
) -> Result<(), String> {
	if ctx
		.db
		.foreign_account()
		.id()
		.find(reference.to_string())
		.is_some()
	{
		return Err(format!(
			"Foreign account {reference} is already registered in the system.",
		));
	}

	let new_account = ctx.db.foreign_account().insert(ForeignAccount {
		id: reference.to_string(),
		callsign,
		owner_id: ctx.identity(),

		profile_id: Some(
			ctx.db
				.account_profile()
				.insert(AccountProfile {
					id:       0,
					metadata: metadata.unwrap_or_default(),
				})
				.id,
		),
	});

	let mut root_account = ctx.identity().try_resolve(ctx)?;

	root_account.foreign_account_ownership.push(new_account.id);

	ctx.db.native_account().id().update(NativeAccount {
		updated_at: ctx.timestamp,
		..root_account
	});

	Ok(())
}

#[reducer]
/// Updates the local representation
/// of a 3rd party platform account handle / username.
pub fn update_foreign_account_callsign(
	ctx: &ReducerContext, reference: ForeignAccountReference, callsign: Option<String>,
) -> Result<(), String> {
	let account = ctx
		.db
		.foreign_account()
		.id()
		.find(reference.to_string())
		.ok_or(format!(
			"Foreign account {reference} is not registered in the system."
		))?;

	ctx.db.foreign_account().id().update(ForeignAccount {
		callsign,
		..account
	});

	Ok(())
}

#[reducer]
/// Updates the local representation of a 3rd party platform account profile.
pub fn update_foreign_account_profile(
	ctx: &ReducerContext, reference: ForeignAccountReference,
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

	ctx.db.foreign_account().id().update(ForeignAccount {
		profile_id: Some(profile.id),
		..account
	});

	Ok(())
}
