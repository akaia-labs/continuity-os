use spacetimedb::{Identity, SpacetimeType, Timestamp, table};

pub type AccountId = Identity;

#[derive(PartialEq, SpacetimeType)]
pub enum AccountRole {
	Admin,
	Interactor,
}

#[table(name = account, public)]
pub struct Account {
	#[primary_key]
	pub id: AccountId,
	pub callsign: Option<String>,
	pub role: AccountRole,
	pub is_online: bool,
	pub created_at: Timestamp,
	pub updated_at: Timestamp,
	pub last_seen_at: Timestamp,
}
