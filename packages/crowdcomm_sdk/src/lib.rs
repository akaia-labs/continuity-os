mod common;
pub mod configuration;
pub mod integrations;

pub use common::*;

pub mod singularity {
	pub use singularity_client::{
		common::*,
		domain::{entities::*, features::*, intersections::*},
	};
}
