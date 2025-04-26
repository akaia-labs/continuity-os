use crate::common::bindings::telegram::{self, command::BotCommands, *};

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
pub enum Command {
	/// Display this text.
	#[command(aliases = ["h", "?"])]
	Help,
}

pub async fn on_command(
	bot: telegram::Bot, msg: telegram::Message, cmd: Command,
) -> ResponseResult<()> {
	match cmd {
		| Command::Help => {
			if let Some(message_thread_id) = msg.thread_id {
				bot.send_message(msg.chat.id, Command::descriptions().to_string())
					.message_thread_id(message_thread_id)
					.await?
			} else {
				bot.send_message(msg.chat.id, Command::descriptions().to_string())
					.await?
			}
		},
	};

	Ok(())
}
