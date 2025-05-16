use crate::common::stdb::{AccountProfile, RemoteDbContext};

pub trait ProfileRetrieval {
	/// Retrieves the associated profile, if present.
	fn profile(&self, ctx: &impl RemoteDbContext) -> Option<AccountProfile>;

	/// Retrieves the associated local profile, if present.
	///
	/// For **native accounts** this should be equivalent to [`Self::profile`].
	///
	/// For **foreign accounts owned by native accounts**, the profile
	/// of the owner account should take precedence.
	fn local_profile(&self, ctx: &impl RemoteDbContext) -> Option<AccountProfile>;
}
