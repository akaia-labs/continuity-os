mod config;
mod direct;
mod id;
mod metadata;
mod primary;
mod reducers;
mod standalone;
mod subordinate;

use spacetimedb::SpacetimeType;

pub use self::{metadata::*, primary::*};

#[derive(SpacetimeType, Clone)]
pub enum ChannelKind {
	Standalone,
	Primary,
	Subordinate,
}
