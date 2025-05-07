use std::{pin::Pin, sync::Arc};

use crowcomm::{
	crowd_core::{DbConnection, Message},
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
) -> impl Fn(Bot, Message, UserCommand) -> Pin<Box<dyn Future<Output = Result<(), RequestError>> + Send>>
{
	move |bot: Bot, msg: Message, cmd: UserCommand| {
		let ctx = core_ctx.clone();
		let user = msg.from();

		if let Some(user) = user {
			match cmd {
				| UserCommand::MyAccountId => {
					if let Some(message_thread_id) = msg.thread_id {
						bot.send_message(msg.chat.id, msg.from.unwrap().id.to_string())
							.message_thread_id(message_thread_id)
							.await?
					} else {
						bot.send_message(msg.chat.id, msg.from.unwrap().id.to_string())
							.await?
					}
				},
			};

			respond(())
		} else {
			Box::pin(async move { respond(()) })
		}
	}
}
