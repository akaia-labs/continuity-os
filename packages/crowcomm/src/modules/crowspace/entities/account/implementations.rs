use std::fmt::{self, Display, Formatter};

use super::AccountDisplayName;
use crate::crowspace::{self, PublicProfileName, PublicProfileTableAccess};

impl Display for PublicProfileName {
	fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
		if let Some(name_extension) = &self.name_extension {
			write!(formatter, "{} {}", self.short_name, name_extension)
		} else {
			write!(formatter, "{}", self.short_name)
		}
	}
}

impl AccountDisplayName for crowspace::Account {
	fn get_display_name(&self, ctx: &impl crowspace::RemoteDbContext) -> String {
		ctx.db()
			.public_profile()
			.id()
			.find(&self.profile_id)
			.map(|profile| profile.metadata.name.to_string())
			.unwrap_or_else(|| self.callsign.clone())
	}
}
