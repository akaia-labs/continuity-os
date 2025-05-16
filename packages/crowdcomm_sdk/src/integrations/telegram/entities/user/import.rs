use teloxide::types::User;

use crate::corvidx::{
	AccountProfileMetadata, AccountProfileName, ForeignAccountReference, ForeignPlatformTag,
	foreign_account::ForeignAccountImport, profile::ProfileImport,
};

impl ForeignAccountImport for User {
	fn into_account_reference(&self) -> ForeignAccountReference {
		ForeignAccountReference {
			id:           self.id.to_string(),
			platform_tag: ForeignPlatformTag::Telegram,
		}
	}
}

impl ProfileImport for User {
	fn into_profile_metadata(&self) -> AccountProfileMetadata {
		AccountProfileMetadata {
			name: AccountProfileName {
				short_name:     self.first_name.clone(),
				name_extension: self.last_name.clone(),
			},

			// TODO: Implement bio retrieval
			bio: "".to_string(),
		}
	}
}
