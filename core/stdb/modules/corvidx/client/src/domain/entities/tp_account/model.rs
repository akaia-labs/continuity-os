use crate::common::stdb::TpAccountReference;

/// "{String}@{TpPlatformTag}"
pub type TpAccountId = String;

impl TpAccountReference {
	pub const DELIMITER: char = '@';
}
