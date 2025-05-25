use crate::{
	common::stdb::RemoteDbContext,
	domain::entities::external_platform::SupportedExternalActorOrigin,
};

pub trait PlatformAssociation<T> {
	fn platform_association(
		&self, ctx: &impl RemoteDbContext, platform_tag: SupportedExternalActorOrigin,
	) -> Option<T>;
}
