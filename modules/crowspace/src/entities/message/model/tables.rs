use spacetimedb::{Identity, Timestamp, table};

#[table(name = message, public)]
pub struct Message {
	#[auto_inc]
	#[primary_key]
	pub id: u64,
	pub sender: Identity,
	pub sent: Timestamp,
	pub text: String,
}
