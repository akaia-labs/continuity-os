use std::sync::Arc;

use corvutils::{print_error, print_success};
use crowdcomm_sdk::{
	corvidx::stdb::{DbConnection, resolve_account_link_request},
	integrations::{commands::AccountLinkRequestAction, dtos::ActionCommand},
};

use crate::BotInstanceType;

pub fn handle_request_callback(
	ctx: Arc<DbConnection>, bot: BotInstanceType, command: ActionCommand<AccountLinkRequestAction>,
) {
	match command.payload {
		| AccountLinkRequestAction::Accept(id) => {
			let result = ctx.reducers.resolve_account_link_request(id, true);

			if result.is_ok() {
				print_success(format!("Account link request {id} has been accepted."));
			} else if let Err(err) = result {
				print_error(format!("Failed to accept account link request {id}: {err}"));
			}
		},

		| AccountLinkRequestAction::Reject(id) => {
			let result = ctx.reducers.resolve_account_link_request(id, false);

			if result.is_ok() {
				print_success(format!("Account link request {id} has been rejected."));
			} else if let Err(err) = result {
				print_error(format!("Failed to reject account link request {id}: {err}"));
			}
		},
	}
}
