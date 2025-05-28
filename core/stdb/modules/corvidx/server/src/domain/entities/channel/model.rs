mod config;
mod metadata;
mod primary;
mod standalone;
mod subordinate;

use spacetimedb::SpacetimeType;

pub type ChannelId = String;

#[derive(SpacetimeType, Clone)]
pub enum ChannelKind {
	/// Maps to `"type": null` in Matrix
	Default,

	/// Maps to `"type": "m.space"` in Matrix
	SuperChannel,
}
