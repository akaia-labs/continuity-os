pub use teloxide_core::types::{
	Message as TelegramMessage, Update as TelegramUpdate, User as TelegramUser,
};

pub use crate::singularity::stdb::Message as SingularityMessage;
use crate::singularity::stdb::{ActorProfileMetadata, EventContext, ExternalActorReference};

pub trait ExternalActorIdentification {
	/// Derives locally recognized reference from third-party actor's properties
	fn into_actor_ref(&self) -> ExternalActorReference;
}

pub trait ProfileImport {
	/// Puts third-party profile into locally recognized format
	fn into_actor_profile_metadata(&self) -> ActorProfileMetadata;
}

pub trait SingularityUpdateHandler<EventType> {
	fn handle(&self, context: &EventContext, event: &EventType);
}
