use std::fmt::{self, Display, Formatter};

use spacetimedb::{SpacetimeType, table};

pub type ActorProfileId = i128;

#[table(name = actor_profile, public)]
pub struct ActorProfile {
	#[auto_inc]
	#[primary_key]
	pub id:       ActorProfileId,
	pub metadata: ActorProfileMetadata,
}

#[derive(SpacetimeType)]
/// Logical grouping of name tokens
pub struct ActorProfileName {
	pub short_name:     String,
	pub name_extension: Option<String>,
}

impl Display for ActorProfileName {
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
pub struct ActorProfileMetadata {
	pub name:        ActorProfileName,
	/// Markdown-formatted string
	pub description: String,
}

impl Default for ActorProfileMetadata {
	fn default() -> Self {
		ActorProfileMetadata {
			name: ActorProfileName {
				short_name:     "Anonymous".to_string(),
				name_extension: None,
			},

			description: "".to_string(),
		}
	}
}

impl ActorProfileMetadata {
	pub fn default_with_name(name: String) -> Self {
		ActorProfileMetadata {
			name: ActorProfileName {
				short_name:     name,
				name_extension: None,
			},

			..Default::default()
		}
	}
}
