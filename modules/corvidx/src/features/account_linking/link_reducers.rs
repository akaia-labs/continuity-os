use std::time::Duration;

use spacetimedb::{ReducerContext, Table, reducer};

use super::tables::{AccountLinkRequest, account_link_request};
use crate::{
	common::traits::AsRecordResolver,
	entities::foreign_account::{ForeignAccount, ForeignAccountReference, foreign_account},
};

const LINK_REQUEST_TIMEOUT: Duration = Duration::from_secs(30 * 60);

#[reducer]
/// Binds a foreign account to a native account.
pub fn link_foreign_account(
	ctx: &ReducerContext, reference: ForeignAccountReference,
) -> Result<(), String> {
	let native_account = ctx.sender.resolve(ctx)?;

	let foreign_account = ctx
		.db
		.foreign_account()
		.id()
		.find(reference.to_string())
		.ok_or(format!(
			"Foreign account {reference} is not registered in the system."
		))?;

	ctx.db.foreign_account().id().update(ForeignAccount {
		owner_id: Some(native_account.id),
		..foreign_account
	});

	Ok(())
}

#[reducer]
/// Creates a foreign to native account link request.
pub fn create_account_link_request(
	ctx: &ReducerContext, reference: ForeignAccountReference,
) -> Result<(), String> {
	let foreign_account = reference.resolve(ctx)?;
	let native_account = ctx.sender.resolve(ctx)?;

	ctx.db.account_link_request().insert(AccountLinkRequest {
		id:                   0,
		created_at:           ctx.timestamp,
		requester_account_id: native_account.id,
		subject_account_id:   foreign_account.id,

		expires_at: ctx
			.timestamp
			.checked_add(LINK_REQUEST_TIMEOUT.into())
			.unwrap(),
	});

	Ok(())
}
