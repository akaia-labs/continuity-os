use crowdcomm_sdk::{
	configuration::corvid_subsystem, singularity::external_platform::SupportedExternalActorOrigin,
};

pub const ROOT_SUBSYSTEM_CANONICAL_NAME: &str = corvid_subsystem::CANONICAL_NAME;

pub const SERVICE_CANONICAL_NAME: &str = "Telecrow";

pub const TARGET_FOREIGN_PLATFORM_TAG: SupportedExternalActorOrigin =
	SupportedExternalActorOrigin::Telegram;
