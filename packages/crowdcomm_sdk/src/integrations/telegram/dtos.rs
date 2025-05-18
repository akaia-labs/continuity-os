use teloxide_core::types::{ChatId, ThreadId};

pub struct OutboundTelegramMessage {
	pub chat_id:   ChatId,
	pub thread_id: Option<ThreadId>,
	pub text:      String,
}
