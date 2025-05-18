use crowdcomm_sdk::{
	configuration::corvid_subsystem_config, corvidx::tp_platform::SupportedTpPlatformTag,
};

pub const ROOT_SUBSYSTEM_CANONICAL_NAME: &str = corvid_subsystem_config::CANONICAL_NAME;

pub const SERVICE_CANONICAL_NAME: &str = "Telecrow";

pub const TARGET_FOREIGN_PLATFORM_TAG: SupportedTpPlatformTag =
	SupportedTpPlatformTag::Telegram;
