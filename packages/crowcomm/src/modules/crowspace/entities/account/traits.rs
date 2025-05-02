use crate::crowspace;

pub trait AccountDisplayName {
	fn get_display_name(&self, ctx: &impl crowspace::RemoteDbContext) -> String;
}
