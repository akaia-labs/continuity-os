pub mod corvid;

pub use corvid as corvid_subsystem;

pub struct SubsystemModuleConfig {
	pub module_name: String,
}

pub struct SubsystemServiceConfig {
	pub auth_token:                   String,
	/// Corresponds to the ID of the third-party-platform-bound structure
	/// controlled exclusively by the subsystem owner community.
	/// e.g. for Telegram, this would be a group / supergroup ID.
	pub delegated_authority_space_id: String,
}
