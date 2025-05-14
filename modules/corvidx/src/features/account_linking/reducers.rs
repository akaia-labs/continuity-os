use std::time::Duration;

use corvutils::StringExtensions;
use spacetimedb::{ReducerContext, Table, reducer};

use super::tables::{AccountLinkRequest, AccountLinkRequestExpirySchedule, account_link_request};
use crate::{
	common::traits::AsRecordResolver,
	entities::foreign_account::{ForeignAccount, ForeignAccountReference, foreign_account},
	features::account_linking::tables::account_link_request_schedule,
};

const LINK_REQUEST_TIMEOUT: Duration = Duration::from_secs(5 * 60);

#[reducer]
/// Creates a foreign to native account link request.
pub fn create_account_link_request(
	ctx: &ReducerContext, reference: ForeignAccountReference,
) -> Result<(), String> {
	let native_account = ctx.sender.resolve(ctx)?;
	let foreign_account = reference.resolve(ctx)?;

	let request = ctx.db.account_link_request().insert(AccountLinkRequest {
		id:                   0,
		created_at:           ctx.timestamp,
		requester_account_id: native_account.id,
		subject_account_id:   foreign_account.id,

		expires_at: ctx
			.timestamp
			.checked_add(LINK_REQUEST_TIMEOUT.into())
			.ok_or(format!(
				"Unable to calculate account link request expiration date for {reference}."
			))?,
	});

	ctx.db
		.account_link_request_schedule()
		.insert(AccountLinkRequestExpirySchedule {
			scheduled_id: 0,
			scheduled_at: request.expires_at.into(),
			request_id:   request.id,
		});

	log::info!(
		"{requester} created an account link request {id} for foreign account {reference}.",
		requester = native_account.id,
		id = request.id,
	);

	Ok(())
}

#[reducer]
/// Binds a foreign account to a native account.
pub fn link_foreign_account(
	ctx: &ReducerContext, reference: ForeignAccountReference,
) -> Result<(), String> {
	let native_account = ctx.sender.resolve(ctx)?;
	let foreign_account = reference.resolve(ctx)?;

	ctx.db.foreign_account().id().update(ForeignAccount {
		owner_id: Some(native_account.id),
		..foreign_account
	});

	Ok(())
}

#[reducer]
/// Unbinds a foreign account from a native account.
pub fn unlink_foreign_account(
	ctx: &ReducerContext, reference: ForeignAccountReference,
) -> Result<(), String> {
	let native_account = ctx.sender.resolve(ctx)?;
	let foreign_account = reference.resolve(ctx)?;

	if foreign_account.owner_id.is_some() && foreign_account.owner_id.unwrap() != native_account.id
	{
		return Err(format!(
			"Account {id} is not linked to the foreign account {reference}.",
			id = ctx.sender,
		));
	}

	ctx.db.foreign_account().id().update(ForeignAccount {
		owner_id: None,
		..foreign_account
	});

	Ok(())
}

#[reducer]
/// Removes an account link request.
/// Should only be invoked via a scheduled task.
pub fn scheduled_delete_account_link_request(
	ctx: &ReducerContext, args: AccountLinkRequestExpirySchedule,
) -> Result<(), String> {
	if ctx.sender != ctx.identity() {
		return Err(r#"
			Reducer `scheduled_delete_account_link_request`
			may not be invoked by clients, only via scheduling.
		"#
		.to_string()
		.squash_whitespace());
	}

	ctx.db.account_link_request().id().delete(args.request_id);
	log::info!("Account link request {} expired.", args.request_id);

	Ok(())
}
