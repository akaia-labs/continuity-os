use spacetimedb::{Identity, ReducerContext, SpacetimeType, Timestamp, table};

use crate::{
	common::traits::RecordResolver,
	entities::{account_profile::AccountProfileId, foreign_account::ForeignAccountId},
};

pub type NativeAccountId = Identity;

#[derive(PartialEq, SpacetimeType)]
pub enum NativeAccountLocalRole {
	Service,
	Admin,
	Interactor,
}

#[table(name = native_account, public)]
pub struct NativeAccount {
	#[primary_key]
	pub id: NativeAccountId,

	#[unique]
	#[index(btree)]
	/// An authentic counterpart to "username" or "handle" on other platforms.
	pub callsign: String,

	#[index(btree)]
	pub role: NativeAccountLocalRole,

	pub is_online:    bool,
	pub created_at:   Timestamp,
	pub updated_at:   Timestamp,
	pub last_seen_at: Timestamp,

	#[unique]
	#[index(btree)]
	pub profile_id: AccountProfileId,

	pub foreign_accounts: Vec<ForeignAccountId>,
}

impl RecordResolver<NativeAccount> for NativeAccountId {
	fn resolve(&self, ctx: &ReducerContext) -> Result<NativeAccount, String> {
		ctx.db.native_account().id().find(ctx.sender).ok_or(format!(
			"Identity {id} does not have an account.",
			id = ctx.sender
		))
	}
}
