use std::{future::Future, pin::Pin, sync::Arc};

use crowdcomm_sdk::{
	corvidx::stdb::{DbConnection, ExternalActorTableAccess},
	integrations::ports::ExternalActorIdentification,
};
use teloxide::{
	RequestError, payloads::SendMessageSetters, prelude::Requester,
	sugar::request::RequestReplyExt, types::Message, utils::command::BotCommands,
};

use crate::{BotInstanceType, common::constants::ROOT_SUBSYSTEM_CANONICAL_NAME};

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
pub enum PrivateCommand {
	#[command()]
	/// ℹ️ Display id of the Corvi.d account record
	/// associated with your Telegram account.
	MyAccountId,
}

pub fn private_handler(
	ctx: Arc<DbConnection>,
) -> impl Fn(
	BotInstanceType,
	Message,
	PrivateCommand,
) -> Pin<Box<dyn Future<Output = Result<(), RequestError>> + Send>> {
	move |bot: BotInstanceType, msg: Message, cmd: PrivateCommand| {
		let ctx = ctx.clone();
		let user = msg.from;

		Box::pin(async move {
			if !msg.chat.is_private() {
				let error_response_text = "This command can only be used as a DM to the bot.";

				if let Some(message_thread_id) = msg.thread_id {
					bot.send_message(msg.chat.id, error_response_text)
						.message_thread_id(message_thread_id)
						.reply_to(msg.id)
						.await?
				} else {
					bot.send_message(msg.chat.id, error_response_text).await?
				};

				return Ok(());
			}

			if let Some(user) = user {
				let external_actor = ctx
					.db
					.external_actor()
					.id()
					.find(&user.into_exref().to_string());

				match cmd {
					| PrivateCommand::MyAccountId => {
						let response_text = if let Some(external_actor) = external_actor {
							format!("Your account id is <code>{}</code>", external_actor.id)
						} else {
							format!(
								r#"
									Your Telegram account is not registered
									in this {ROOT_SUBSYSTEM_CANONICAL_NAME} instance.
								"#,
							)
						};

						if let Some(message_thread_id) = msg.thread_id {
							bot.send_message(msg.chat.id, response_text)
								.message_thread_id(message_thread_id)
								.reply_to(msg.id)
								.await?
						} else {
							bot.send_message(msg.chat.id, response_text)
								.reply_to(msg.id)
								.await?
						};
					},
				};
			}

			Ok(())
		})
	}
}
