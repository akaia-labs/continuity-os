use teloxide_core::types::{ChatId, InlineKeyboardMarkup, MessageId, ThreadId};

#[derive(Debug)]
pub struct OutboundTelegramMessage {
	pub chat_id:             ChatId,
	pub thread_id:           Option<ThreadId>,
	pub reply_to_message_id: Option<MessageId>,
	pub text:                String,
}

#[derive(Debug)]
pub struct OutboundTelegramActionRequest {
	pub chat_id:             ChatId,
	pub thread_id:           Option<ThreadId>,
	pub reply_to_message_id: Option<MessageId>,
	pub text:                String,
	pub reply_markup:        InlineKeyboardMarkup,
}
