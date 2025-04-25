use std::env;

pub struct CrowtocolModuleConfig {
	pub name: String,
}

pub struct CrowtocolModules {
	pub chat: CrowtocolModuleConfig,
}

pub struct CrowtocolEnvConfig {
	pub host: String,
	pub modules: CrowtocolModules,
}

pub fn get_env_config() -> Option<CrowtocolEnvConfig> {
	if env::var("CROWD_HOST").is_ok() && env::var("CROWD_CHAT_MODULE_NAME").is_ok() {
		Some(CrowtocolEnvConfig {
			host: env::var("CROWD_HOST").unwrap(),

			modules: CrowtocolModules {
				chat: CrowtocolModuleConfig {
					name: env::var("CROWD_CHAT_MODULE_NAME").unwrap(),
				},
			},
		})
	} else {
		None
	}
}
