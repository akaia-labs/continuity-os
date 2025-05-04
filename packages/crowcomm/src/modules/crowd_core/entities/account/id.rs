use std::fmt::{self, Display, Formatter};

use crate::crowd_core::ForeignAccountReference;

// TODO: figure out how to reduce the reimplementation overhead

impl ForeignAccountReference {
	pub const DELIMITER: char = '@';
}

impl Display for ForeignAccountReference {
	fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
		write!(
			formatter,
			"{}{}{}",
			self.id,
			Self::DELIMITER,
			// ! Temporarily hardcoded
			"telegram".to_string()
		)
	}
}
