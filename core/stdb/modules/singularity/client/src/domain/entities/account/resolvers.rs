use super::AccountId;
use crate::common::{
	ports::{ProfileResolution, RecordResolver},
	stdb::{Account, AccountTableAccess, ActorProfile, ActorProfileTableAccess, RemoteDbContext},
};

impl ProfileResolution for Account {
	/// Resolves an account profile
	fn profile(&self, ctx: &impl RemoteDbContext) -> Option<ActorProfile> {
		ctx.db().actor_profile().id().find(&self.profile)
	}

	/// Equivalent to `.profile`
	fn root_profile(&self, ctx: &impl RemoteDbContext) -> Option<ActorProfile> {
		self.profile(ctx)
	}
}

// TODO: Implement try_resolve
impl RecordResolver<Account> for AccountId {
	/// Resolves a internal account by ID
	fn resolve(&self, ctx: &impl RemoteDbContext) -> Option<Account> {
		ctx.db().account().id().find(&self)
	}
}
