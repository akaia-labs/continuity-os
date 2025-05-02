use spacetimedb::{Identity, Timestamp, table};

#[table(name = service, public)]
pub struct Service {
	pub name: Identity,
	pub registered: Timestamp,
}
