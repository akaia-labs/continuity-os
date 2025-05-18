use super::TpAccountId;
use crate::common::{
	ports::{ProfileResolution, RecordResolution},
	stdb::{
		AccountProfile, AccountProfileTableAccess, TpAccount, TpAccountReference,
		TpAccountTableAccess, NativeAccountTableAccess, RemoteDbContext,
	},
};

impl ProfileResolution for TpAccount {
	/// Resolves a third-party account profile
	fn profile(&self, ctx: &impl RemoteDbContext) -> Option<AccountProfile> {
		if let Some(profile_id) = self.profile_id {
			ctx.db().account_profile().id().find(&profile_id)
		} else {
			None
		}
	}

	/// Walks the ownership tree starting from the bound internal account
	/// (if present) to retrieve the first available account profile
	fn native_profile(&self, ctx: &impl RemoteDbContext) -> Option<AccountProfile> {
		if let Some(owner) = ctx.db().native_account().id().find(&self.owner_id) {
			owner.native_profile(ctx)
		} else if let Some(profile_id) = self.profile_id {
			ctx.db().account_profile().id().find(&profile_id)
		} else {
			None
		}
	}
}

impl RecordResolution<TpAccount> for TpAccountReference {
	/// Resolves a third-party account by its reference
	fn resolve(&self, ctx: &impl RemoteDbContext) -> Option<TpAccount> {
		ctx.db().tp_account().id().find(&self.to_string())
	}
}

impl RecordResolution<TpAccount> for TpAccountId {
	/// Resolves a third-party account by ID
	fn resolve(&self, ctx: &impl RemoteDbContext) -> Option<TpAccount> {
		ctx.db().tp_account().id().find(&self)
	}
}
