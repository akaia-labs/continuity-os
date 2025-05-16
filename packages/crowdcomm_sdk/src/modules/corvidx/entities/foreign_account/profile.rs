use crate::corvidx::{
	AccountProfile, AccountProfileTableAccess, ForeignAccount, NativeAccountTableAccess,
	RemoteDbContext,
	profile::ProfileRetrieval,
	traits::{DisplayName, Displayable},
};

impl ProfileRetrieval for ForeignAccount {
	fn profile(&self, ctx: &impl RemoteDbContext) -> Option<AccountProfile> {
		if let Some(profile_id) = self.profile_id {
			ctx.db().account_profile().id().find(&profile_id)
		} else {
			None
		}
	}

	fn local_profile(&self, ctx: &impl RemoteDbContext) -> Option<AccountProfile> {
		if let Some(owner) = ctx.db().native_account().id().find(&self.owner_id) {
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
