use spacetimedb::{Identity, Timestamp, table};

#[table(name = user, public)]
pub struct User {
	#[primary_key]
	pub identity: Identity,
	pub name: Option<String>,
	pub is_online: bool,
	pub updated_at: Timestamp,
	pub last_seen_at: Timestamp,
}
