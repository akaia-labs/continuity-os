mod runtime_env;

use self::runtime_env::CorvidSubsystemRuntimeEnvConfig;
use super::{SubsystemModuleConfig, SubsystemServiceConfig};

pub const CANONICAL_NAME: &str = "ContinuityOS";

pub fn get_config() -> ContinuitySystemConfig {
	let CorvidSubsystemRuntimeEnvConfig {
		modulehost,
		services,
	} = runtime_env::get_config();

	ContinuitySystemConfig {
		module_host: modulehost,

		components: CorvidSubsystemComponents {
			singularity: SubsystemModuleConfig {
				module_name: "singularity".to_string(),
			},

			// TODO: Make subsystem services optional here as well as in env
			telecrow: SubsystemServiceConfig {
				auth_token:                   services.telecrow.authtkn,
				delegated_authority_space_id: services.telecrow.dasid,
			},
		},
	}
}

pub struct ContinuitySystemConfig {
	pub module_host: String,
	pub components:  CorvidSubsystemComponents,
}

pub struct CorvidSubsystemComponents {
	pub singularity: SubsystemModuleConfig,
	pub telecrow:    SubsystemServiceConfig,
}
