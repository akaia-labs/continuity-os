use crate::crowspace;

pub trait Displayable {
	fn display_name(&self) -> String;
}
pub trait DisplayName {
	fn display_name(&self, ctx: &impl crowspace::RemoteDbContext) -> String;
}
