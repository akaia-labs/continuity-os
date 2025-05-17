use crate::common::{
	ports::ProfileResolution,
	presentation::{DisplayName, Displayable},
	stdb::{NativeAccount, RemoteDbContext},
};

impl DisplayName for NativeAccount {
	/// Returns the display name of the linked profile, if present,
	/// otherwise the account callsign
	fn display_name(&self, ctx: &impl RemoteDbContext) -> String {
		self.profile(ctx)
			.map(|p| p.display_name())
			.unwrap_or(self.callsign.clone())
	}
}
