use std::sync::Arc;

use corvutils::{print_error, print_success};
use crowdcomm_sdk::{
	corvidx::stdb::{DbConnection, resolve_external_authentication_request},
	integrations::{commands::ExtAuthReqResolution, dtos::ActionCommand, ports::TelegramUser},
};
use teloxide::{prelude::Requester, types::MaybeInaccessibleMessage};

use crate::BotInstanceType;

pub async fn handle_command(
	ctx: Arc<DbConnection>, bot: BotInstanceType, prompt_msg: Option<MaybeInaccessibleMessage>,
	command: ActionCommand<ExtAuthReqResolution>, caller: TelegramUser,
) {
	match command.payload {
		| ExtAuthReqResolution::Accept(id) => {
			let result = ctx
				.reducers
				.resolve_external_authentication_request(id, true);

			if result.is_ok() {
				let success_msg = format!("Account link request {id} has been accepted.");
				print_success(success_msg.clone());

				if let Err(err) = bot.send_message(caller.id, success_msg).await {
					print_error(format!("Failed to send message: {err}"));
				};
			} else if let Err(err) = result {
				print_error(format!("Failed to accept account link request {id}: {err}"));
			}
		},

		| ExtAuthReqResolution::Reject(id) => {
			let result = ctx
				.reducers
				.resolve_external_authentication_request(id, false);

			if result.is_ok() {
				let success_msg_text = format!("Account link request {id} has been rejected.");
				print_success(success_msg_text.clone());

				if let Err(err) = bot.send_message(caller.id, success_msg_text).await {
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
