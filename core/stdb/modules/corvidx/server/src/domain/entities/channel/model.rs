mod config;
mod metadata;
mod primary;
mod standalone;
mod subordinate;

use spacetimedb::SpacetimeType;

pub use self::{primary::*, standalone::*, subordinate::*};

#[derive(SpacetimeType, Clone)]
pub enum ChannelKind {
	Standalone,
	Primary,
	Subordinate,
}
