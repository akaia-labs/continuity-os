use crate::common::{
	bindings::telegram::{self, command::BotCommands, *},
	runtime,
};

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
pub enum BasicCommand {
	/// Display this text.
	#[command(aliases = ["h", "?"])]
	Help,
}

pub async fn on_basic_command(
	bot: telegram::Bot, msg: telegram::Message, cmd: BasicCommand,
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
