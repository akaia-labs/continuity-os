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
			singularity: SubsystemModuleConfig {
				db_name: modules.core.dbname,
			},

			// TODO: Make subsystem services optional here as well as in env
			telecrow: SubsystemServiceConfig {
				auth_token:                   services.telecrow.authtkn,
				delegated_authority_space_id: services.telecrow.dasid,
			},
		},
	}
}

pub struct CorvidSubsystemConfig {
	pub module_host: String,
	pub components:  CorvidSubsystemComponents,
}

pub struct CorvidSubsystemComponents {
	pub singularity:  SubsystemModuleConfig,
	pub telecrow: SubsystemServiceConfig,
}
