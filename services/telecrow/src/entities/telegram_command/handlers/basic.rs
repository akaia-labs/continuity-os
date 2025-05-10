use corvutils::ListFormat;
use teloxide::{
	payloads::SendMessageSetters,
	prelude::{Requester, ResponseResult},
	sugar::request::RequestReplyExt,
	types::{ChatKind, Message, PublicChatKind},
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

	#[command()]
	/// Get basic information about the current chat
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
						"Basic commands:".to_string(),
						BasicCommand::descriptions().to_string(),
						"Private (DM-only) commands:".to_string(),
						PrivateCommand::descriptions().to_string(),
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
			let chat_type = match &msg.chat.kind {
				| ChatKind::Private(_) => "DM",

				| ChatKind::Public(props) => match &props.kind {
					| PublicChatKind::Channel(_) => "Channel",
					| PublicChatKind::Group => "Group",

					| PublicChatKind::Supergroup(supergroup_props) => supergroup_props
						.is_forum
						.then(|| "Forum")
						.unwrap_or("Supergroup"),
				},
			};

			let message_request = bot
				.send_message(
					msg.chat.id,
					vec![
						format!("Chat type: <code>{chat_type}</code>"),
						format!("Chat title: {}", msg.chat.title().unwrap_or("not set")),
						format!("Chat ID: {}", msg.chat.id),
						format!("Chat handle: {}", msg.chat.username().unwrap_or("not set")),
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
	};

	Ok(())
}
