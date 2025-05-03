use crate::crowspace::{
	self, AccountTableAccess, PublicProfileTableAccess,
	traits::{DisplayName, Displayable},
};

impl DisplayName for crowspace::Account {
	/// Returns the display name of the linked profile, if present,
	/// otherwise the account callsign
	fn display_name(&self, ctx: &impl crowspace::RemoteDbContext) -> String {
		ctx.db()
			.public_profile()
			.id()
			.find(&self.profile_id)
			.map(|p| p.display_name())
			.unwrap_or(self.callsign.clone())
	}
}

impl DisplayName for crowspace::ExternalAccount {
	/// Walks the ownership tree starting from the bound internal account
	/// (if present) to retrieve the first available identifier for display
	fn display_name(&self, ctx: &impl crowspace::RemoteDbContext) -> String {
		let owner_account = if let Some(owner_id) = self.owner_id {
			ctx.db().account().id().find(&owner_id)
		} else {
			None
		};

		if let Some(owner) = owner_account {
			owner.display_name(ctx)
		} else if let Some(profile_id) = self.profile_id {
			ctx.db()
				.public_profile()
				.id()
				.find(&profile_id)
				.map(|p| p.display_name())
				.unwrap_or(self.id.clone())
		} else {
			self.id.clone()
		}
	}
}
