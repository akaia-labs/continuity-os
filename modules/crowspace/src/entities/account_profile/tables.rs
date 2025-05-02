use spacetimedb::{SpacetimeType, table};

use crate::entities::{external_account::ExternalAccountId, internal_account::AccountId};

pub type AccountProfileId = u64;

#[table(name = account_profile, public)]
pub struct AccountProfile {
	#[auto_inc]
	#[primary_key]
	pub id:       AccountProfileId,
	pub owner_id: AccountProfileOwnerId,
	pub metadata: AccountProfileMetadata,
}

#[derive(SpacetimeType)]
pub enum AccountProfileOwnerId {
	InternalAccountId(AccountId),
	ExternalAccountId(ExternalAccountId),
}

#[derive(SpacetimeType, serde::Serialize, serde::Deserialize)]
/// Logical grouping of name tokens
pub struct AccountProfileName {
	#[serde(default = "default_short_name")]
	pub short_name:     String,
	pub name_extension: Option<String>,
}

fn default_short_name() -> String {
	"Anonymous".to_string()
}

impl Default for AccountProfileName {
	fn default() -> Self {
		Self {
			short_name:     default_short_name(),
			name_extension: None,
		}
	}
}

#[derive(SpacetimeType, serde::Serialize, serde::Deserialize, Default)]
/// Logical grouping of name tokens
pub struct AccountProfileMetadata {
	pub name: AccountProfileName,
	#[serde(default)]
	/// Markdown-formatted string
	pub bio:  String,
}
