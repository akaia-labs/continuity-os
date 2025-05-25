use super::stdb::{ActorProfile, RemoteDbContext};

pub trait RecordResolution<RecordType> {
	fn resolve(&self, ctx: &impl RemoteDbContext) -> Option<RecordType>;
}

pub trait ProfileResolution {
	/// Retrieves the associated profile, if present.
	fn profile(&self, ctx: &impl RemoteDbContext) -> Option<ActorProfile>;

	/// Retrieves the associated local profile, if present.
	///
	/// For **internal accounts** this should be equivalent to [`Self::profile`].
	///
	/// For **third-party accounts owned by internal accounts**, the profile
	/// of the owner account should take precedence.
	fn native_profile(&self, ctx: &impl RemoteDbContext) -> Option<ActorProfile>;
}
