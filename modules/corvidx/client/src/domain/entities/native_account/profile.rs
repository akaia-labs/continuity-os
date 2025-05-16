use crate::{
	common::{
		stdb::{AccountProfile, AccountProfileTableAccess, NativeAccount, RemoteDbContext},
		traits::{DisplayName, Displayable},
	},
	entities::account_profile::ProfileRetrieval,
};

impl ProfileRetrieval for NativeAccount {
	fn profile(&self, ctx: &impl RemoteDbContext) -> Option<AccountProfile> {
		ctx.db().account_profile().id().find(&self.profile_id)
	}

	fn local_profile(&self, ctx: &impl RemoteDbContext) -> Option<AccountProfile> {
		self.profile(ctx)
	}
}

impl DisplayName for NativeAccount {
	/// Returns the display name of the linked profile, if present,
	/// otherwise the account callsign
	fn display_name(&self, ctx: &impl RemoteDbContext) -> String {
		self.profile(ctx)
			.map(|p| p.display_name())
			.unwrap_or(self.callsign.clone())
	}
}
