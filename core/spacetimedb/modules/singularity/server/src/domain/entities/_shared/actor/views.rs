use std::fmt::Display;

use crate::domain::entities::shared::keys::ActorId;

impl Display for ActorId {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			| ActorId::Internal(id) => write!(f, "{id}"),
			| ActorId::External(id) => write!(f, "{id}"),
		}
	}
}
