use spacetimedb::table;

pub type ChannelConfigId = i128;

//* WIP
// TODO: Consider introducing versioning in the future
#[table(name = channel_config, public)]
pub struct ChannelConfig {
	#[auto_inc]
	#[primary_key]
	pub id: ChannelConfigId,

	/// Whether Matrix federation is allowed (`m.federate`)
	pub m_federate: bool,

	/// `m.room.join_rules`: "public" | "invite" | "knock" | …
	pub join_rule: Option<String>,

	/// `m.room.history_visibility`
	pub history_visibility: Option<String>,

	/// `m.room.guest_access`
	pub guest_access: Option<String>,

	/// `m.room.encryption.algorithm`
	pub encryption_algo: Option<String>,
}
