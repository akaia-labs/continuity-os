use teloxide_core::types::{ChatId, MessageId, ThreadId};

#[derive(Debug)]
pub struct OutboundTelegramMessage {
	pub chat_id:             ChatId,
	pub thread_id:           Option<ThreadId>,
	pub reply_to_message_id: Option<MessageId>,
	pub text:                String,
}
