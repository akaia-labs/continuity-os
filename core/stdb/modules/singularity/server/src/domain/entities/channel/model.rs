mod config;
mod direct;
mod id;
mod metadata;
mod primary;
mod reducers;
mod standalone;
mod subordinate;

use spacetimedb::SpacetimeType;

#[allow(unused_imports)]
pub use self::{direct::*, metadata::*, primary::*, standalone::*, subordinate::*};

#[derive(SpacetimeType, Clone)]
pub enum ChannelKind {
	Standalone,
	Primary,
	Subordinate,
}
