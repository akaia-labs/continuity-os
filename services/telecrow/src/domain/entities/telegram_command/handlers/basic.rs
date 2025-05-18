use corvutils::ListFormat;
use crowdcomm_sdk::presentation::Summary;
use teloxide::{
	payloads::SendMessageSetters,
	prelude::{Requester, ResponseResult},
	sugar::request::RequestReplyExt,
	types::Message,
	utils::command::BotCommands,
};

use super::{super::CommandDescriptionsFormat, PrivateCommand};
use crate::BotInstanceType;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
pub enum BasicCommand {
	#[command(aliases = ["h", "?"])]
	/// ℹ️ Display this text.
	Help,

	#[command()]
	/// ℹ️ Get basic information about the current chat
	ChatInfo,
}

pub async fn on_basic_command(
	bot: BotInstanceType, msg: Message, cmd: BasicCommand,
) -> ResponseResult<()> {
	match cmd {
		| BasicCommand::Help => {
			let message_request = bot
				.send_message(
					msg.chat.id,
					vec![
						"⚙️ Basic commands:".to_string(),
						BasicCommand::descriptions().format_list(),
						"🔒 Private (DM-only) commands:".to_string(),
						PrivateCommand::descriptions().format_list(),
					]
					.format_list(),
				)
				.reply_to(msg.id);

			if let Some(message_thread_id) = msg.thread_id {
				message_request.message_thread_id(message_thread_id).await?
			} else {
				message_request.await?
			}
		},

		| BasicCommand::ChatInfo => {
			let message_request = bot
				.send_message(msg.chat.id, msg.chat.summary())
				.reply_to(msg.id);

			if let Some(message_thread_id) = msg.thread_id {
				message_request.message_thread_id(message_thread_id).await?
			} else {
				message_request.await?
			}
		},
	};

	Ok(())
}
