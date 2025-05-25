use crate::common::stdb::ExternalActorReference;

/// "{String}@{ExternalPlatformTag}"
pub type ExternalActorId = String;

impl ExternalActorReference {
	pub const DELIMITER: char = '@';
}
