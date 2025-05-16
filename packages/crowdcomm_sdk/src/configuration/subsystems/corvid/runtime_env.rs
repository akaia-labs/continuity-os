use corvutils::StringExtensions;
use serde::Deserialize;
use serde_env::from_env_with_prefix;

use super::{super::SubsystemModulesRuntimeEnvConfig, CANONICAL_NAME};

const ENV_PREFIX: &str = "CORVID";

pub fn get_config() -> CorvidSubsystemRuntimeEnvConfig {
	let error_message = format!(
		r#"
			Unable to retrieve environment variables for {CANONICAL_NAME} subsystem.
			Check out .env.example reference and ensure
			your environment variables are set correctly.
		"#,
	)
	.squash_whitespace();

	from_env_with_prefix(ENV_PREFIX).expect(error_message.as_str())
}

#[derive(Debug, Deserialize)]
pub struct CorvidSubsystemRuntimeEnvConfig {
	pub modulehost: String,
	pub modules:    SubsystemModulesRuntimeEnvConfig,
	pub services:   CorvidServicesRuntimeEnvConfig,
}

#[derive(Debug, Deserialize)]
pub struct CorvidServicesRuntimeEnvConfig {
	pub telecrow: CorvidCoreServiceRuntimeEnvConfig,
}

#[derive(Debug, Deserialize)]
pub struct CorvidCoreServiceRuntimeEnvConfig {
	pub authtkn: String,
	/// A shorthand for Delegated Authority Space ID.
	/// Corresponds to the ID of the foreign-platform-bound structure
	/// controlled exclusively by the subsystem owner community.
	pub dasid:   String,
}
