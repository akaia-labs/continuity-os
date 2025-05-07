use std::{future::Future, pin::Pin, sync::Arc};

use crowcomm::{
	crowd_core::{DbConnection, ForeignAccountTableAccess, account::ForeignAccountImport},
	telegram,
};
use teloxide::{
	Bot, RequestError,
	payloads::SendMessageSetters,
	prelude::{Requester, ResponseResult},
	utils::command::BotCommands,
};

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
pub enum BasicCommand {
	#[command(aliases = ["h", "?"])]
	/// Display this text.
	Help,
}

pub async fn on_basic_command(
	bot: Bot, msg: telegram::Message, cmd: BasicCommand,
) -> ResponseResult<()> {
	match cmd {
		| BasicCommand::Help => {
			if let Some(message_thread_id) = msg.thread_id {
				bot.send_message(msg.chat.id, BasicCommand::descriptions().to_string())
					.message_thread_id(message_thread_id)
					.await?
			} else {
				bot.send_message(msg.chat.id, BasicCommand::descriptions().to_string())
					.await?
			}
		},
	};

	Ok(())
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
pub enum UserCommand {
	#[command(aliases = ["myaccountid"])]
	/// Display id of the foreign account
	/// associated with the caller's Telegram account.
	MyAccountId,
}

pub fn user_handler(
	core_ctx: Arc<DbConnection>,
) -> impl Fn(
	Bot,
	telegram::Message,
	UserCommand,
) -> Pin<Box<dyn Future<Output = Result<(), RequestError>> + Send>> {
	move |bot: Bot, msg: telegram::Message, cmd: UserCommand| {
		let ctx = core_ctx.clone();
		let user = msg.from;

		Box::pin(async move {
			if let Some(user) = user {
				// Here you can interact with the database via ctx before replying
				// For example, look up user information, update preferences, etc.
				// Example: let user_data = ctx.db.some_table().find_by_id(user.id.0).await;

				let foreign_account = ctx
					.db
					.foreign_account()
					.id()
					.find(&user.into_account_reference().to_string())
					.ok_or(format!("Foreign account is not registered in the system."));

				match cmd {
					| UserCommand::MyAccountId => {
						// You can use ctx here to perform database operations
						// before responding to the user

						// Now send the response
						if let Some(message_thread_id) = msg.thread_id {
							bot.send_message(msg.chat.id, user.id.to_string())
								.message_thread_id(message_thread_id)
								.await?
						} else {
							bot.send_message(msg.chat.id, user.id.to_string()).await?
						}
					},
				};
			}

			Ok(())
		})
	}
}
