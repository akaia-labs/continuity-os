use crate::common::stdb::ExternalActorReference;

/// "{String}@{ExternalActorOrigin}"
pub type ExternalActorId = String;

impl ExternalActorReference {
	pub const DELIMITER: char = '@';
}
