use std::time::Duration;

use capitalize::Capitalize;
use corvutils::StringExtensions;
use spacetimedb::{ReducerContext, Table, reducer};

use super::tables::{
	AccountLinkRequest, AccountLinkRequestExpirySchedule, AccountLinkRequestId,
	account_link_request,
};
use crate::{
	common::ports::RecordResolution,
	domain::{
		entities::{
			foreign_account::{ForeignAccount, ForeignAccountReference, foreign_account},
			message::{Message, MessageAuthorId, message},
			native_account::native_account,
		},
		features::account_linking::tables::account_link_request_schedule,
	},
};

const LINK_REQUEST_TIMEOUT: Duration = Duration::from_secs(5 * 60);

// TODO Implement rate limit
#[reducer]
/// Creates a foreign to native account link request.
pub fn create_account_link_request(
	ctx: &ReducerContext, reference: ForeignAccountReference,
) -> Result<(), String> {
	let foreign_account = reference.try_resolve(ctx)?;

	if foreign_account.owner_id != ctx.identity() {
		return Err(format!(
			"Foreign account {reference} is already linked to another native account.",
		));
	}

	let native_account = ctx.sender.try_resolve(ctx)?;

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
pub fn resolve_account_link_request(
	ctx: &ReducerContext, request_id: AccountLinkRequestId, is_approved: bool,
) -> Result<(), String> {
	let request = request_id.try_resolve(ctx)?;

	let AccountLinkRequest {
		requester_account_id,
		subject_account_id,
		..
	} = &request;

	if is_approved {
		let mut native_account = requester_account_id.try_resolve(ctx)?;
		let foreign_account = subject_account_id.try_resolve(ctx)?;

		ctx.db.foreign_account().id().update(ForeignAccount {
			owner_id: native_account.id,
			..foreign_account
		});

		native_account
			.foreign_account_ownership
			.push(subject_account_id.to_string());

		ctx.db.native_account().id().update(native_account);
	}

	ctx.db.account_link_request().id().delete(request_id);
	report_account_link_resolution(ctx, request, is_approved);

	Ok(())
}

#[reducer]
/// Unbinds a foreign account from a native account.
pub fn unlink_foreign_account(
	ctx: &ReducerContext, reference: ForeignAccountReference,
) -> Result<(), String> {
	let mut native_account = ctx.sender.try_resolve(ctx)?;
	let foreign_account = reference.try_resolve(ctx)?;

	if foreign_account.owner_id != native_account.id {
		return Err(format!(
			"Account {id} is not linked to the foreign account {reference}.",
			id = ctx.sender,
		));
	}

	ctx.db.foreign_account().id().update(ForeignAccount {
		owner_id: ctx.identity(),
		..foreign_account
	});

	native_account
		.foreign_account_ownership
		.retain(|id| id != &reference.to_string());

	ctx.db.native_account().id().update(native_account);

	Ok(())
}

#[reducer]
/// Reports account link resolution outcome.
pub fn report_account_link_resolution(
	ctx: &ReducerContext, request: AccountLinkRequest, is_approved: bool,
) {
	let AccountLinkRequest {
		requester_account_id: _,
		subject_account_id,
		..
	} = request;

	let display_account_reference = subject_account_id
		.parse::<ForeignAccountReference>()
		.map_or(subject_account_id, |far| {
			format!(
				"{platform_name} account {fa_id}",
				platform_name = far.platform_tag.to_string().capitalize(),
				fa_id = far.id,
			)
		});

	// TODO: Send DM instead, once DMs are implemented
	let result = ctx.db.message().try_insert(Message {
		id:        0,
		sender:    ctx.identity(),
		sent_at:   ctx.timestamp,
		author_id: MessageAuthorId::NativeAccountId(ctx.identity()),

		text: if is_approved {
			format!("{display_account_reference} has been linked to your account.")
		} else {
			format!("Account link request for {display_account_reference} has been rejected.")
		},
	});

	if let Err(err) = result {
		log::error!("Failed to send account link resolution message: {err}");
	}
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
