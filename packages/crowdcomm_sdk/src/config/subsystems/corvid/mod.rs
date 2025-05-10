mod runtime_env;

use self::runtime_env::CorvidSubsystemRuntimeEnvConfig;
use super::{SubsystemModuleConfig, SubsystemServiceConfig};

pub const CANONICAL_NAME: &str = "Corvi.d";

pub fn get() -> CorvidSubsystemConfig {
	let CorvidSubsystemRuntimeEnvConfig {
		modulehost,
		modules,
		services,
	} = runtime_env::get_config();

	CorvidSubsystemConfig {
		module_host: modulehost,

		components: CorvidSubsystemComponents {
			corvidx: SubsystemModuleConfig {
				db_name: modules.core.dbname,
			},

			// TODO: Make it optional here as well as in env
			telecrow: SubsystemServiceConfig {
				auth_token:                   services.telecrow.authtkn,
				initial_authorized_entity_id: services.telecrow.iaeid,
			},
		},
	}
}

pub struct CorvidSubsystemConfig {
	pub module_host: String,
	pub components:  CorvidSubsystemComponents,
}

pub struct CorvidSubsystemComponents {
	pub corvidx:  SubsystemModuleConfig,
	pub telecrow: SubsystemServiceConfig,
}
