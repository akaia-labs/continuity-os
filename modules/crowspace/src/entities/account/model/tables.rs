use spacetimedb::{Identity, Timestamp, table};

#[table(name = account, public)]
pub struct Account {
	#[primary_key]
	pub identity: Identity,
	pub callsign: Option<String>,
	pub is_online: bool,
	pub updated_at: Timestamp,
	pub last_seen_at: Timestamp,
}
