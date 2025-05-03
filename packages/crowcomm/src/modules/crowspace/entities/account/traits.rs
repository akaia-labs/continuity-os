use crate::crowspace;

pub trait AccountDisplayName {
	fn display_name(&self, ctx: &impl crowspace::RemoteDbContext) -> String;
}
