use teloxide_core::types::{ChatId, MessageId, ThreadId};

pub struct OutboundTelegramMessage {
	pub chat_id:   ChatId,
	pub thread_id: Option<ThreadId>,
	pub reply_to:  Option<MessageId>,
	pub text:      String,
}
