use spacetimedb::{SpacetimeType, table};

use crate::entities::{external_account::ExternalAccountId, internal_account::AccountId};

pub type SocialProfileId = u64;

#[table(name = social_profile, public)]
pub struct SocialProfile {
	#[auto_inc]
	#[primary_key]
	pub id:               SocialProfileId,
	pub owner_account_id: AccountId,
	pub metadata:         SocialProfileMetadata,
}

#[derive(SpacetimeType)]
pub enum SocialProfileOwnerId {
	InternalAccountId(AccountId),
	ExternalAccountId(ExternalAccountId),
}

#[derive(SpacetimeType, serde::Serialize, serde::Deserialize)]
/// Logical grouping of name tokens
pub struct SocialProfileName {
	#[serde(default = "default_short_name")]
	pub short_name:     String,
	pub name_extension: Option<String>,
}

fn default_short_name() -> String {
	"Anonymous".to_string()
}

#[derive(SpacetimeType, serde::Serialize, serde::Deserialize)]
/// Logical grouping of name tokens
pub struct SocialProfileMetadata {
	pub name: SocialProfileName,
	#[serde(default)]
	/// Markdown-formatted string
	pub bio:  String,
}
