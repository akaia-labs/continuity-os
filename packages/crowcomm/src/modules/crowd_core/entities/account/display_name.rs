use crate::crowd_core::{
	AccountProfileTableAccess, ForeignAccount, LocalAccount, LocalAccountTableAccess,
	RemoteDbContext,
	traits::{DisplayName, Displayable},
};

impl DisplayName for LocalAccount {
	/// Returns the display name of the linked profile, if present,
	/// otherwise the account callsign
	fn display_name(&self, ctx: &impl RemoteDbContext) -> String {
		ctx.db()
			.account_profile()
			.id()
			.find(&self.profile_id)
			.map(|p| p.display_name())
			.unwrap_or(self.callsign.clone())
	}
}

impl DisplayName for ForeignAccount {
	/// Walks the ownership tree starting from the bound internal account
	/// (if present) to retrieve the first available identifier for display
	fn display_name(&self, ctx: &impl RemoteDbContext) -> String {
		let owner_account = if let Some(owner_id) = self.owner_id {
			ctx.db().local_account().id().find(&owner_id)
		} else {
			None
		};

		if let Some(owner) = owner_account {
			owner.display_name(ctx)
		} else if let Some(profile_id) = self.profile_id {
			ctx.db()
				.account_profile()
				.id()
				.find(&profile_id)
				.map(|p| p.display_name())
				.unwrap_or(self.id.clone())
		} else {
			self.id.clone()
		}
	}
}
