use spacetimedb::{Identity, SpacetimeType, Timestamp, table};

use crate::entities::account_profile::AccountProfileId;

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
	pub id:           NativeAccountId,
	#[unique]
	#[index(btree)]
	/// An authentic counterpart to "username" or "handle" on other platforms.
	pub callsign:     String,
	#[index(btree)]
	pub role:         NativeAccountLocalRole,
	pub is_online:    bool,
	pub created_at:   Timestamp,
	pub updated_at:   Timestamp,
	pub last_seen_at: Timestamp,
	#[unique]
	#[index(btree)]
	pub profile_id:   AccountProfileId,
}
