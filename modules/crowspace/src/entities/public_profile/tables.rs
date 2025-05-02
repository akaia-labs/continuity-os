use std::fmt::{self, Display, Formatter};

use spacetimedb::{SpacetimeType, table};

use crate::entities::{external_account::ExternalAccountId, internal_account::AccountId};

pub type PublicProfileId = u64;

#[table(name = public_profile, public)]
pub struct PublicProfile {
	#[auto_inc]
	#[primary_key]
	pub id:       PublicProfileId,
	#[unique]
	#[index(btree)]
	pub owner_id: PublicProfileOwnerId,
	pub metadata: PublicProfileMetadata,
}

#[derive(SpacetimeType)]
pub enum PublicProfileOwnerId {
	InternalAccountId(AccountId),
	ExternalAccountId(ExternalAccountId),
}

#[derive(SpacetimeType, Default, serde::Serialize, serde::Deserialize)]
/// Logical grouping of name tokens
pub struct PublicProfileName {
	#[serde(default = "default_short_name")]
	pub short_name:     String,
	pub name_extension: Option<String>,
}

fn default_short_name() -> String {
	"Anonymous".to_string()
}

impl Display for PublicProfileName {
	fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
		if let Some(name_extension) = &self.name_extension {
			write!(formatter, "{} {}", self.short_name, name_extension)
		} else {
			write!(formatter, "{}", self.short_name)
		}
	}
}

#[derive(SpacetimeType, Default, serde::Serialize, serde::Deserialize)]
/// Logical grouping of name tokens
pub struct PublicProfileMetadata {
	pub name: PublicProfileName,
	#[serde(default)]
	/// Markdown-formatted string
	pub bio:  String,
}

impl PublicProfileMetadata {
	pub fn default_with_name(name: String) -> Self {
		PublicProfileMetadata {
			name: PublicProfileName {
				short_name:     name,
				name_extension: None,
			},

			..Default::default()
		}
	}
}
