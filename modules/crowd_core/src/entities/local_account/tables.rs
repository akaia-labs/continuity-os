use spacetimedb::{Identity, SpacetimeType, Timestamp, table};

use crate::entities::account_profile::AccountProfileId;

pub type LocalAccountId = Identity;

#[derive(PartialEq, SpacetimeType)]
pub enum LocalAccountRole {
	Service,
	Admin,
	Interactor,
}

#[table(name = local_account, public)]
pub struct LocalAccount {
	#[primary_key]
	pub id:           LocalAccountId,
	#[unique]
	#[index(btree)]
	pub callsign:     String,
	#[index(btree)]
	pub role:         LocalAccountRole,
	pub is_online:    bool,
	pub created_at:   Timestamp,
	pub updated_at:   Timestamp,
	pub last_seen_at: Timestamp,
	#[unique]
	#[index(btree)]
	pub profile_id:   AccountProfileId,
}
