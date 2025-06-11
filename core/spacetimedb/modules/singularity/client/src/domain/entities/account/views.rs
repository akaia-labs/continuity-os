use crate::common::{
	ports::ProfileResolution,
	presentation::{DisplayName, Displayable},
	stdb::{Account, RemoteDbContext},
};

impl DisplayName for Account {
	/// Returns the display name of the linked profile, if present,
	/// otherwise the account callsign
	fn display_name(&self, ctx: &impl RemoteDbContext) -> String {
		self.profile(ctx)
			.map(|p| p.display_name())
			.unwrap_or(self.callsign.clone())
	}
}
