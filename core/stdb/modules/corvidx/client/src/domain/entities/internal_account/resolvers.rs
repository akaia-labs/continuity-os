use super::AccountId;
use crate::common::{
	ports::{ProfileResolution, RecordResolution},
	stdb::{
		ActorProfile, ActorProfileTableAccess, Account, AccountTableAccess,
		RemoteDbContext,
	},
};

impl ProfileResolution for Account {
	/// Resolves a third-party account profile
	fn profile(&self, ctx: &impl RemoteDbContext) -> Option<ActorProfile> {
		ctx.db().account_profile().id().find(&self.profile_id)
	}

	/// Equivalent to `.profile` for internal accounts
	fn native_profile(&self, ctx: &impl RemoteDbContext) -> Option<ActorProfile> {
		self.profile(ctx)
	}
}

// TODO: Implement try_resolve
impl RecordResolution<Account> for AccountId {
	/// Resolves a internal account by ID
	fn resolve(&self, ctx: &impl RemoteDbContext) -> Option<Account> {
		ctx.db().account().id().find(&self)
	}
}
