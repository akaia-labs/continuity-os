use std::fmt::{self, Display, Formatter};

use spacetimedb::{SpacetimeType, table};

pub type AccountProfileId = u64;

#[table(name = account_profile, public)]
pub struct AccountProfile {
	#[auto_inc]
	#[primary_key]
	pub id:       AccountProfileId,
	pub metadata: AccountProfileMetadata,
}

#[derive(SpacetimeType)]
/// Logical grouping of name tokens
pub struct AccountProfileName {
	pub short_name:     String,
	pub name_extension: Option<String>,
}

impl Display for AccountProfileName {
	fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
		if let Some(name_extension) = &self.name_extension {
			write!(formatter, "{} {}", self.short_name, name_extension)
		} else {
			write!(formatter, "{}", self.short_name)
		}
	}
}

#[derive(SpacetimeType)]
/// Logical grouping of name tokens
pub struct AccountProfileMetadata {
	pub name: AccountProfileName,
	/// Markdown-formatted string
	pub bio:  String,
}

impl Default for AccountProfileMetadata {
	fn default() -> Self {
		AccountProfileMetadata {
			name: AccountProfileName {
				short_name:     "Anonymous".to_string(),
				name_extension: None,
			},

			bio: "".to_string(),
		}
	}
}

impl AccountProfileMetadata {
	pub fn default_with_name(name: String) -> Self {
		AccountProfileMetadata {
			name: AccountProfileName {
				short_name:     name,
				name_extension: None,
			},

			..Default::default()
		}
	}
}
