use std::fmt::{self, Display, Formatter};

use crate::common::{
	ports::ProfileResolution,
	presentation::{DisplayName, Displayable},
	stdb::{ExternalActor, ExternalActorReference, RemoteDbContext},
};

impl DisplayName for ExternalActor {
	/// Walks the ownership tree starting from the bound internal account
	/// (if present) to retrieve the first available identifier for display
	fn display_name(&self, ctx: &impl RemoteDbContext) -> String {
		if let Some(native_profile) = self.native_profile(ctx) {
			native_profile.display_name()
		} else if let Some(profile) = self.profile(ctx) {
			profile.display_name()
		} else {
			self.id.clone()
		}
	}
}

impl Display for ExternalActorReference {
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
