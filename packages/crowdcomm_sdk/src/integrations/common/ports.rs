pub use teloxide_core::types::{
	Message as TelegramMessage, Update as TelegramUpdate, User as TelegramUser,
};

pub use crate::corvidx::stdb::Message as CorvidxMessage;
use crate::corvidx::stdb::{ActorProfileMetadata, EventContext, ExternalActorReference};

pub trait ExternalActorImport {
	/// Puts third-party account data into locally recognized format
	fn into_account_reference(&self) -> ExternalActorReference;
}

pub trait ProfileImport {
	/// Puts third-party profile into locally recognized format
	fn into_profile_metadata(&self) -> ActorProfileMetadata;
}

pub trait CorvidxEventHandler<EventType> {
	fn handle(&self, context: &EventContext, event: &EventType);
}
