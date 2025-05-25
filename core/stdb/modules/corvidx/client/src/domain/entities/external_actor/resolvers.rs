use super::ExternalActorId;
use crate::common::{
	ports::{ProfileResolution, RecordResolution},
	stdb::{
		ActorProfile, ActorProfileTableAccess, AccountTableAccess, RemoteDbContext,
		ExternalActor, ExternalActorReference, ExternalActorTableAccess,
	},
};

impl ProfileResolution for ExternalActor {
	/// Resolves a third-party account profile
	fn profile(&self, ctx: &impl RemoteDbContext) -> Option<ActorProfile> {
		if let Some(profile_id) = self.profile_id {
			ctx.db().account_profile().id().find(&profile_id)
		} else {
			None
		}
	}

	/// Walks the ownership tree starting from the bound internal account
	/// (if present) to retrieve the first available account profile
	fn native_profile(&self, ctx: &impl RemoteDbContext) -> Option<ActorProfile> {
		if let Some(owner) = self
			.owner_id
			.and_then(|id| ctx.db().account().id().find(&id))
		{
			owner.native_profile(ctx)
		} else if let Some(profile_id) = self.profile_id {
			ctx.db().account_profile().id().find(&profile_id)
		} else {
			None
		}
	}
}

impl RecordResolution<ExternalActor> for ExternalActorReference {
	/// Resolves a third-party account by its reference
	fn resolve(&self, ctx: &impl RemoteDbContext) -> Option<ExternalActor> {
		ctx.db().external_actor().id().find(&self.to_string())
	}
}

impl RecordResolution<ExternalActor> for ExternalActorId {
	/// Resolves a third-party account by ID
	fn resolve(&self, ctx: &impl RemoteDbContext) -> Option<ExternalActor> {
		ctx.db().external_actor().id().find(&self)
	}
}
