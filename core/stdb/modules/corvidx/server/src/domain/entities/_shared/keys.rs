//! Facilitates establishing relationships between entities in the DB,
//! allowing reusing the same type as both primary and foreign key
//! without cross-references between entity modules.

use spacetimedb::Identity;

/// Primary key for the account table
pub type AccountId = Identity;

/// Primary key for the external actor table
///
/// Must convey the following format:
/// `"{String}@{ExternalActorOrigin}"`
pub type ExternalActorId = String;
