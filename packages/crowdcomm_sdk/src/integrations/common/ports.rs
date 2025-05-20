use corvidx_client::common::stdb::{AccountProfileMetadata, EventContext, TpAccountReference};
pub use teloxide_core::types::{Message as TelegramMessage, Update as TelegramUpdate};

pub use crate::corvidx::stdb::Message as CorvidxMessage;

pub trait TpAccountImport {
	/// Puts third-party account data into locally recognized format
	fn into_account_reference(&self) -> TpAccountReference;
}

pub trait ProfileImport {
	/// Puts third-party profile into locally recognized format
	fn into_profile_metadata(&self) -> AccountProfileMetadata;
}

pub trait CorvidxEventHandler<EventType> {
	fn handle(&self, context: &EventContext, event: &EventType);
}
