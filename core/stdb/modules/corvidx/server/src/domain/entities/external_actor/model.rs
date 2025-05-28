use capitalize::Capitalize;
use spacetimedb::{DbContext, ReducerContext, table};

use crate::{
	common::ports::RecordResolution,
	domain::entities::shared::actor::{
		ActorProfileId, ExternalActorId, ExternalActorReference, ExternalActorReferenceParseErr,
		InternalActorId,
	},
};

#[table(name = external_actor, public)]
/// Locally recognized format for third-party accounts
pub struct ExternalActor {
	#[primary_key]
	/// "{String}@{ExternalActorOrigin}"
	pub id: ExternalActorId,

	#[index(btree)]
	/// Holds username, handle, or any other identifier
	/// with the similar meaning, if present.
	pub callsign: Option<String>,

	#[index(btree)]
	pub account: Option<InternalActorId>,

	#[unique]
	#[index(btree)]
	pub profile: Option<ActorProfileId>,
}

impl RecordResolution<ExternalActor> for ExternalActorId {
	fn try_resolve(&self, ctx: &ReducerContext) -> Result<ExternalActor, String> {
		let ExternalActorReference {
			id: external_author_id,
			origin,
		} = self
			.parse()
			.map_err(|e: ExternalActorReferenceParseErr| e.to_string())?;

		ctx.db().external_actor().id().find(self).ok_or(format!(
			"{platform_name} account {external_author_id} is not registered in the system.",
			platform_name = origin.to_string().capitalize(),
		))
	}
}

impl RecordResolution<ExternalActor> for ExternalActorReference {
	fn try_resolve(&self, ctx: &ReducerContext) -> Result<ExternalActor, String> {
		self.to_string().try_resolve(ctx)
	}
}
