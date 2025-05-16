pub mod corvid;

pub use corvid as corvid_subsystem_config;
use serde::Deserialize;

pub struct SubsystemModuleConfig {
	pub db_name: String,
}

pub struct SubsystemServiceConfig {
	pub auth_token:                   String,
	/// Corresponds to the ID of the foreign-platform-bound structure
	/// controlled exclusively by the subsystem owner community.
	/// e.g. for Telegram, this would be a group / supergroup ID.
	pub delegated_authority_space_id: String,
}

#[derive(Debug, Deserialize)]
struct SubsystemModulesRuntimeEnvConfig {
	core: SubsystemCoreModuleRuntimeEnvConfig,
}

#[derive(Debug, Deserialize)]
struct SubsystemCoreModuleRuntimeEnvConfig {
	dbname: String,
}
