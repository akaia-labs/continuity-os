use crowcomm::telegram;
use teloxide::{
	payloads::SendMessageSetters,
	prelude::{Requester, ResponseResult},
	sugar::request::RequestReplyExt,
	utils::command::BotCommands,
};

use super::PrivateCommand;
use crate::BotInstanceType;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
pub enum BasicCommand {
	#[command(aliases = ["h", "?"])]
	/// Display this text.
	Help,
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
						"Basic commands:\n\n{}\n\nPrivate (DM-only) commands:\n\n{}",
						BasicCommand::descriptions(),
						PrivateCommand::descriptions()
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
