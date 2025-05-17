use crate::common::{
	ports::ProfileResolution,
	stdb::{AccountProfile, AccountProfileTableAccess, NativeAccount, RemoteDbContext},
};

impl ProfileResolution for NativeAccount {
	fn profile(&self, ctx: &impl RemoteDbContext) -> Option<AccountProfile> {
		ctx.db().account_profile().id().find(&self.profile_id)
	}

	fn local_profile(&self, ctx: &impl RemoteDbContext) -> Option<AccountProfile> {
		self.profile(ctx)
	}
}
