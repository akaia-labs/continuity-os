mod adapters;

use crowdcomm_sdk::corvidx::stdb::DbConnection;

pub use self::adapters::{inbound::*, outbound::*};
use crate::{common::clients::corvidx_client, domain::entities::corvidx_message};

pub fn on_init(corvidx: &DbConnection) {
	corvidx_client::subscribe(&corvidx);
	corvidx_message::inspect(&corvidx);
}
