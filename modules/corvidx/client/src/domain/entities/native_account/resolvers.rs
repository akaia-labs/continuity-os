use super::NativeAccountId;
use crate::common::{
	ports::{ProfileResolution, RecordResolution},
	stdb::{
		AccountProfile, AccountProfileTableAccess, NativeAccount, NativeAccountTableAccess,
		RemoteDbContext,
	},
};

impl ProfileResolution for NativeAccount {
	/// Resolves a third-party account profile
	fn profile(&self, ctx: &impl RemoteDbContext) -> Option<AccountProfile> {
		ctx.db().account_profile().id().find(&self.profile_id)
	}

	/// Equivalent to `.profile` for native accounts
	fn native_profile(&self, ctx: &impl RemoteDbContext) -> Option<AccountProfile> {
		self.profile(ctx)
	}
}

// TODO: Implement try_resolve
impl RecordResolution<NativeAccount> for NativeAccountId {
	/// Resolves a native account by ID
	fn resolve(&self, ctx: &impl RemoteDbContext) -> Option<NativeAccount> {
		ctx.db().native_account().id().find(&self)
	}
}
