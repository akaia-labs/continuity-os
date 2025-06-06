mod adapters;

use crowdcomm_sdk::singularity::stdb::DbConnection;

pub use self::adapters::{inbound::*, outbound::*};
use crate::{common::clients::singularity_client, domain::entities::message};

pub fn on_init(ctx: &DbConnection) {
	singularity_client::subscribe_to_tables(&ctx);
	message::subscribe_to_singularity(&ctx);
}
