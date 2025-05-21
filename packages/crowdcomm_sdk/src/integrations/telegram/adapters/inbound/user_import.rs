use corvidx_client::common::stdb::{
	AccountProfileMetadata, AccountProfileName, TpAccountReference, TpPlatformTag,
};
use teloxide_core::types::User;

use crate::integrations::ports::{ProfileImport, TpAccountImport};

impl TpAccountImport for User {
	fn into_account_reference(&self) -> TpAccountReference {
		TpAccountReference {
			id:           self.id.to_string(),
			platform_tag: TpPlatformTag::Telegram,
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
