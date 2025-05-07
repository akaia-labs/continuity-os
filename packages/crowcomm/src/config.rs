use std::env;

pub struct CrowcommModuleConfig {
	pub name: String,
}

pub struct CrowcommModules {
	pub crowspace: CrowcommModuleConfig,
}

pub struct CrowcommEnvConfig {
	pub host:    String,
	pub modules: CrowcommModules,
}

pub fn get_env_config() -> Option<CrowcommEnvConfig> {
	if env::var("CROWD_HOST").is_ok() && env::var("CROWD_CORE_MODULE_NAME").is_ok() {
		Some(CrowcommEnvConfig {
			host: env::var("CROWD_HOST").unwrap(),

			modules: CrowcommModules {
				crowspace: CrowcommModuleConfig {
					name: env::var("CROWD_CORE_MODULE_NAME").unwrap(),
				},
			},
		})
	} else {
		None
	}
}

pub const PLATFORM_NAME: &str = "Crow.d";
