use spacetimedb::{Identity, SpacetimeType, Timestamp, table};

use crate::entities::account_profile::AccountProfile;

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
	pub id:           AccountId,
	#[unique]
	#[index(btree)]
	pub callsign:     Option<String>,
	#[index(btree)]
	pub role:         AccountRole,
	pub is_online:    bool,
	pub created_at:   Timestamp,
	pub updated_at:   Timestamp,
	pub last_seen_at: Timestamp,
	pub profile:      AccountProfile,
}
