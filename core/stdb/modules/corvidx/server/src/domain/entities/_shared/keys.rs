//! Facilitates establishing relationships between entities in the DB,
//! allowing reusing the same type as both primary and foreign key
//! without cross-references between entity modules.

use spacetimedb::{Identity, SpacetimeType};

/// Primary key for the account table
pub type AccountId = Identity;

/// Primary key for the external actor table
///
/// Must convey the following format:
/// `"{String}@{ExternalActorOrigin}"`
pub type ExternalActorId = String;

#[derive(SpacetimeType, Clone)]
pub enum ChannelId {
	Standalone(StandaloneChannelId),
	Primary(PrimaryChannelId),
	Subordinate(SubordinateChannelId),
}

pub type StandaloneChannelId = String;

pub type PrimaryChannelId = String;

pub type SubordinateChannelId = String;
