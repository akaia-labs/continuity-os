use super::ForeignAccountId;
use crate::common::{
	ports::{ProfileResolution, RecordResolver},
	stdb::{
		AccountProfile, AccountProfileTableAccess, ForeignAccount, ForeignAccountReference,
		ForeignAccountTableAccess, NativeAccountTableAccess, RemoteDbContext,
	},
};

impl ProfileResolution for ForeignAccount {
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

impl RecordResolver<ForeignAccount> for ForeignAccountReference {
	fn resolve(&self, ctx: &impl RemoteDbContext) -> Option<ForeignAccount> {
		ctx.db().foreign_account().id().find(&self.to_string())
	}
}

// TODO: Might not be much of help, consider removing
impl RecordResolver<ForeignAccount> for ForeignAccountId {
	fn resolve(&self, ctx: &impl RemoteDbContext) -> Option<ForeignAccount> {
		ctx.db().foreign_account().id().find(&self)
	}
}
