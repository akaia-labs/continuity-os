use crate::crowd_core::{
	AccountProfile, AccountProfileTableAccess, ForeignAccount, LocalAccount,
	LocalAccountTableAccess, RemoteDbContext,
	profile::ProfileRetrieval,
	traits::{DisplayName, Displayable},
};

impl ProfileRetrieval for LocalAccount {
	fn profile(&self, ctx: &impl RemoteDbContext) -> Option<AccountProfile> {
		ctx.db().account_profile().id().find(&self.profile_id)
	}

	fn local_profile(&self, ctx: &impl RemoteDbContext) -> Option<AccountProfile> {
		self.profile(ctx)
	}
}

impl DisplayName for LocalAccount {
	/// Returns the display name of the linked profile, if present,
	/// otherwise the account callsign
	fn display_name(&self, ctx: &impl RemoteDbContext) -> String {
		self.profile(ctx)
			.map(|p| p.display_name())
			.unwrap_or(self.callsign.clone())
	}
}

impl ProfileRetrieval for ForeignAccount {
	fn profile(&self, ctx: &impl RemoteDbContext) -> Option<AccountProfile> {
		if let Some(profile_id) = self.profile_id {
			ctx.db().account_profile().id().find(&profile_id)
		} else {
			None
		}
	}

	fn local_profile(&self, ctx: &impl RemoteDbContext) -> Option<AccountProfile> {
		let owner_account = if let Some(owner_id) = self.owner_id {
			ctx.db().local_account().id().find(&owner_id)
		} else {
			None
		};

		if let Some(owner) = owner_account {
			owner.local_profile(ctx)
		} else if let Some(profile_id) = self.profile_id {
			ctx.db().account_profile().id().find(&profile_id)
		} else {
			None
		}
	}
}

impl DisplayName for ForeignAccount {
	/// Walks the ownership tree starting from the bound internal account
	/// (if present) to retrieve the first available identifier for display
	fn display_name(&self, ctx: &impl RemoteDbContext) -> String {
		if let Some(local_profile) = self.local_profile(ctx) {
			local_profile.display_name()
		} else if let Some(profile) = self.profile(ctx) {
			profile.display_name()
		} else {
			self.id.clone()
		}
	}
}
