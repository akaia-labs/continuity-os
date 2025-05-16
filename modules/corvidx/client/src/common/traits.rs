use spacetimedb_sdk::Identity;

use super::stdb::RemoteDbContext;

pub trait Displayable {
	fn display_name(&self) -> String;
}
pub trait DisplayName {
	fn display_name(&self, ctx: &impl RemoteDbContext) -> String;
}

pub trait FromIdentity {
	fn from_identity(ctx: &impl RemoteDbContext, identity: Identity) -> Self;
}
