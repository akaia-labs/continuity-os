mod reducers;

use spacetimedb::{ReducerContext, SpacetimeType, Timestamp, table};

use crate::{
	common::ports::{RecordResolver, Resolvable},
	domain::entities::shared::{
		actor::ActorProfileId,
		keys::{AccountId, ExternalActorId},
	},
};

#[table(name = account, public)]
/// Represents an internal actor.
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
	pub external_actors: Vec<ExternalActorId>,
}

#[derive(PartialEq, SpacetimeType)]
pub enum AccountRole {
	Service,
	Admin,
	Interactor,
}

impl Resolvable for AccountId {
	fn try_is_resolvable(&self, ctx: &ReducerContext) -> Result<(), String> {
		self.try_resolve(ctx).map(|_| ())
	}
}

impl RecordResolver<Account> for AccountId {
	fn try_resolve(&self, ctx: &ReducerContext) -> Result<Account, String> {
		ctx.db
			.account()
			.id()
			.find(self)
			.ok_or(format!("Identity {self} does not have an account.",))
	}
}
