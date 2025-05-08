use std::env;

pub struct CrowdcommModuleConfig {
	pub name: String,
}

pub struct CrowdcommModules {
	pub crowspace: CrowdcommModuleConfig,
}

pub struct CrowdcommEnvConfig {
	pub host:    String,
	pub modules: CrowdcommModules,
}

pub fn get_env_config() -> Option<CrowdcommEnvConfig> {
	if env::var("CROWD_HOST").is_ok() && env::var("CROWD_CORE_MODULE_NAME").is_ok() {
		Some(CrowdcommEnvConfig {
			host: env::var("CROWD_HOST").unwrap(),

			modules: CrowdcommModules {
				crowspace: CrowdcommModuleConfig {
					name: env::var("CROWD_CORE_MODULE_NAME").unwrap(),
				},
			},
		})
	} else {
		None
	}
}

pub const PLATFORM_NAME: &str = "Crow.d";
