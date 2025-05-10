pub mod corvid;

pub use corvid as corvid_subsystem_config;
use serde::Deserialize;

pub struct SubsystemModuleConfig {
	pub db_name: String,
}

pub struct SubsystemServiceConfig {
	pub auth_token:                   String,
	pub initial_authorized_entity_id: String,
}

#[derive(Debug, Deserialize)]
struct SubsystemModulesRuntimeEnvConfig {
	core: SubsystemCoreModuleRuntimeEnvConfig,
}

#[derive(Debug, Deserialize)]
struct SubsystemCoreModuleRuntimeEnvConfig {
	dbname: String,
}
