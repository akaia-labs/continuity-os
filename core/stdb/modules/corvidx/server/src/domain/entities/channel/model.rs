mod config;
mod metadata;
mod primary;
mod standalone;
mod subordinate;

use spacetimedb::SpacetimeType;

#[derive(SpacetimeType, Clone)]
pub enum ChannelKind {
	Standalone,
	Primary,
	Subordinate,
}

#[derive(SpacetimeType, Clone)]
pub enum ChannelId {
	Standalone(StandaloneChannelId),
	Primary(PrimaryChannelId),
	Subordinate(SubordinateChannelId),
}

pub type StandaloneChannelId = String;

pub type PrimaryChannelId = String;

pub type SubordinateChannelId = String;
