use teloxide_core::types::ChatId;

pub struct OutboundTelegramMessage {
	pub chat_id: ChatId,
	pub text:    String,
}
