use crate::common::stdb::ForeignAccountReference;

/// "{String}@{ForeignPlatformTag}"
pub type ForeignAccountId = String;

impl ForeignAccountReference {
	pub const DELIMITER: char = '@';
}
