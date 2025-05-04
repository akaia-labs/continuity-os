use crate::{
	crowd_core::{
		AccountProfileMetadata, AccountProfileName, ForeignAccountReference, ForeignPlatformName,
		account::ForeignAccountImport, profile::ProfileImport,
	},
	telegram,
};

impl ForeignAccountImport for telegram::User {
	fn into_account_reference(&self) -> ForeignAccountReference {
		ForeignAccountReference {
			id:            self.id.to_string(),
			platform_name: ForeignPlatformName::Telegram,
		}
	}
}

impl ProfileImport for telegram::User {
	fn into_profile_metadata(&self) -> AccountProfileMetadata {
		AccountProfileMetadata {
			name: AccountProfileName {
				short_name:     self.first_name.clone(),
				name_extension: self.last_name.clone(),
			},

			// TODO: figure out how to retrieve bio
			bio: "".to_string(),
		}
	}
}
