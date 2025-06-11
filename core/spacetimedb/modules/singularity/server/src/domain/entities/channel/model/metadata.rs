use spacetimedb::SpacetimeType;

#[derive(SpacetimeType, Clone)]
/// Non-indexable additional channel properties.
pub struct ChannelMetadata {
	/// Maps to `m.room.topic`
	pub description: Option<String>,

	/// From `m.room.avatar`
	pub avatar_url: Option<String>,

	/// From `m.room.aliases`
	pub alt_aliases: Option<Vec<String>>,
}

impl Default for ChannelMetadata {
	fn default() -> Self {
		Self {
			description: None,
			avatar_url:  None,
			alt_aliases: None,
		}
	}
}
