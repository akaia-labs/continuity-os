use spacetimedb::{Identity, ReducerContext, SpacetimeType, Timestamp, table};

use crate::{
	common::ports::RecordResolution,
	domain::entities::{actor_profile::ActorProfileId, external_actor::ExternalActorId},
};

pub type AccountId = Identity;

#[derive(PartialEq, SpacetimeType)]
pub enum AccountRole {
	Service,
	Admin,
	Interactor,
}

#[table(name = account, public)]
pub struct Account {
	#[primary_key]
	pub id: AccountId,

	#[unique]
	#[index(btree)]
	/// An authentic counterpart to "username" or "handle" on other platforms.
	pub callsign: String,

	#[index(btree)]
	pub role: AccountRole,

	pub is_online:    bool,
	pub created_at:   Timestamp,
	pub updated_at:   Timestamp,
	pub last_seen_at: Timestamp,

	#[unique]
	#[index(btree)]
	pub profile: ActorProfileId,

	/// Associated external actors
	pub exac_associations: Vec<ExternalActorId>,
}

impl RecordResolution<Account> for AccountId {
	fn try_resolve(&self, ctx: &ReducerContext) -> Result<Account, String> {
		ctx.db
			.account()
			.id()
			.find(self)
			.ok_or(format!("Identity {self} does not have an account.",))
	}
}
