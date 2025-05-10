mod subsystems;

use std::env;

pub use subsystems::corvid_subsystem_config;

pub struct CrowdcommEnvConfig {
	pub host:    String,
	pub modules: CrowdcommCorvidSubsystemComponents,
}

pub struct CrowdcommCorvidSubsystemComponents {
	pub corvidx:  CrowdcommModuleConfig,
	pub telecrow: CrowdcommServiceConfig,
}

pub struct CrowdcommModuleConfig {
	pub name: String,
}

pub struct CrowdcommServiceConfig {
	pub initial_authorized_entity_id: String,
}

pub fn get_env_config() -> Option<CrowdcommEnvConfig> {
	// let result: Vec<(String, String)> = dotenvy::vars()
	// 	.filter(|var_kv| var_kv.0.starts_with("CORVID_"))
	// 	.collect();

	if env::var("CORVID_MODULEHOST").is_ok()
		&& env::var("CORVID_MODULES_CORE_DBNAME").is_ok()
		&& env::var("CORVID_SERVICES_TELECROW_IAEID").is_ok()
	{
		Some(CrowdcommEnvConfig {
			host: env::var("CORVID_MODULEHOST").unwrap(),

			modules: CrowdcommCorvidSubsystemComponents {
				corvidx: CrowdcommModuleConfig {
					name: env::var("CORVID_MODULES_CORE_DBNAME").unwrap(),
				},

				telecrow: CrowdcommServiceConfig {
					initial_authorized_entity_id: env::var("CORVID_SERVICES_TELECROW_IAEID")
						.unwrap(),
				},
			},
		})
	} else {
		None
	}
}
