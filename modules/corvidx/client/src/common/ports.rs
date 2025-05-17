use super::stdb::{AccountProfile, RemoteDbContext};

pub trait RecordResolution<RecordType> {
	fn resolve(&self, ctx: &impl RemoteDbContext) -> Option<RecordType>;
}

pub trait ProfileResolution {
	/// Retrieves the associated profile, if present.
	fn profile(&self, ctx: &impl RemoteDbContext) -> Option<AccountProfile>;

	/// Retrieves the associated local profile, if present.
	///
	/// For **native accounts** this should be equivalent to [`Self::profile`].
	///
	/// For **foreign accounts owned by native accounts**, the profile
	/// of the owner account should take precedence.
	fn native_profile(&self, ctx: &impl RemoteDbContext) -> Option<AccountProfile>;
}
