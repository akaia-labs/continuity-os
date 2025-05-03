use std::fmt::{self, Display, Formatter};

use super::AccountDisplayName;
use crate::crowspace::{self, AccountTableAccess, PublicProfileName, PublicProfileTableAccess};

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
	/// Returns the display name of the linked profile, if present,
	/// otherwise the account callsign
	fn display_name(&self, ctx: &impl crowspace::RemoteDbContext) -> String {
		ctx.db()
			.public_profile()
			.id()
			.find(&self.profile_id)
			.map(|profile| profile.metadata.name.to_string())
			.unwrap_or_else(|| self.callsign.clone())
	}
}

impl AccountDisplayName for crowspace::ExternalAccount {
	/// Walks the ownership tree starting from the bound internal account
	/// (if present) to retrieve the first available identifier for display
	fn display_name(&self, ctx: &impl crowspace::RemoteDbContext) -> String {
		if let Some(owner_id) = self.owner {
			ctx.db()
				.account()
				.id()
				.find(&owner_id)
				.map(|account| account.display_name(ctx))
				.unwrap_or_default()
		} else if let Some(profile_id) = self.profile_id {
			ctx.db()
				.public_profile()
				.id()
				.find(&profile_id)
				.map(|profile| profile.metadata.name.to_string())
				.unwrap_or_else(|| self.id.clone())
		} else {
			self.id.clone()
		}
	}
}
