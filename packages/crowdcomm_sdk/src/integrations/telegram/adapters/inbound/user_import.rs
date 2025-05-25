use corvidx_client::common::stdb::{
	ActorProfileMetadata, ActorProfileName, ExternalActorReference, ExternalActorOrigin,
};
use teloxide_core::types::User;

use crate::integrations::ports::{ExternalActorImport, ProfileImport};

impl ExternalActorImport for User {
	fn into_account_reference(&self) -> ExternalActorReference {
		ExternalActorReference {
			id:           self.id.to_string(),
			platform_tag: ExternalActorOrigin::Telegram,
		}
	}
}

impl ProfileImport for User {
	fn into_profile_metadata(&self) -> ActorProfileMetadata {
		ActorProfileMetadata {
			name: ActorProfileName {
				short_name:     self.first_name.clone(),
				name_extension: self.last_name.clone(),
			},

			// TODO: Implement bio retrieval
			description: "".to_string(),
		}
	}
}
