use std::sync::Arc;

use corvutils::{print_error, print_success};
use crowdcomm_sdk::{
	corvidx::stdb::{DbConnection, resolve_account_link_request},
	integrations::{commands::AccountLinkRequestAction, dtos::ActionCommand, ports::TelegramUser},
};
use teloxide::{prelude::Requester, types::MaybeInaccessibleMessage};

use crate::BotInstanceType;

pub async fn handle_command(
	ctx: Arc<DbConnection>, bot: BotInstanceType, prompt_msg: Option<MaybeInaccessibleMessage>,
	command: ActionCommand<AccountLinkRequestAction>, caller: TelegramUser,
) {
	match command.payload {
		| AccountLinkRequestAction::Accept(id) => {
			let result = ctx.reducers.resolve_account_link_request(id, true);

			if result.is_ok() {
				print_success(format!("Account link request {id} has been accepted."));

				if let Err(err) = bot.send_message(caller.id, format!("Done.")).await {
					print_error(format!("Failed to send message: {err}"));
				};
			} else if let Err(err) = result {
				print_error(format!("Failed to accept account link request {id}: {err}"));
			}
		},

		| AccountLinkRequestAction::Reject(id) => {
			let result = ctx.reducers.resolve_account_link_request(id, false);

			if result.is_ok() {
				print_success(format!("Account link request {id} has been rejected."));

				if let Err(err) = bot.send_message(caller.id, format!("Done.")).await {
					print_error(format!("Failed to send message: {err}"));
				};
			} else if let Err(err) = result {
				print_error(format!("Failed to reject account link request {id}: {err}"));
			}
		},
	}

	if let Some(msg) = prompt_msg {
		let _ = bot.delete_message(caller.id, msg.id()).await;
	}
}
