use std::{future::Future, pin::Pin, sync::Arc};

use crowcomm::{
	PLATFORM_NAME,
	crowd_core::{DbConnection, ForeignAccountTableAccess, account::ForeignAccountImport},
	telegram,
};
use teloxide::{
	RequestError,
	payloads::SendMessageSetters,
	prelude::{Requester, ResponseResult},
	sugar::request::RequestReplyExt,
	types::Message,
	utils::command::BotCommands,
};

use crate::BotInstanceType;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
pub enum BasicCommand {
	#[command(aliases = ["h", "?"])]
	/// Display this text.
	Help,
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "snake_case")]
pub enum DmCommand {
	#[command(aliases = ["myaccountid"])]
	/// Display id of the foreign account
	/// associated with the caller's Telegram account.
	MyAccountId,
}

pub async fn on_basic_command(
	bot: BotInstanceType, msg: telegram::Message, cmd: BasicCommand,
) -> ResponseResult<()> {
	match cmd {
		| BasicCommand::Help => {
			if let Some(message_thread_id) = msg.thread_id {
				bot.send_message(
					msg.chat.id,
					format!(
						"Basic commands:\n\n{}\n\nDM commands:\n\n{}",
						BasicCommand::descriptions(),
						DmCommand::descriptions()
					),
				)
				.message_thread_id(message_thread_id)
				.reply_to(msg.id)
				.await?
			} else {
				bot.send_message(msg.chat.id, BasicCommand::descriptions().to_string())
					.reply_to(msg.id)
					.await?
			}
		},
	};

	Ok(())
}

pub fn dm_handler(
	core_ctx: Arc<DbConnection>,
) -> impl Fn(
	BotInstanceType,
	Message,
	DmCommand,
) -> Pin<Box<dyn Future<Output = Result<(), RequestError>> + Send>> {
	move |bot: BotInstanceType, msg: Message, cmd: DmCommand| {
		let ctx = core_ctx.clone();
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
				let foreign_account = ctx
					.db
					.foreign_account()
					.id()
					.find(&user.into_account_reference().to_string());

				match cmd {
					| DmCommand::MyAccountId => {
						let response_text = if let Some(foreign_account) = foreign_account {
							format!("Your account id is <code>{}</code>", foreign_account.id)
						} else {
							format!(
								"Your Telegram account is not registered in this {PLATFORM_NAME} \
								 instance."
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
