mod adapters;

use crowdcomm_sdk::corvidx::stdb::DbConnection;

pub use self::adapters::{inbound::*, outbound::*};
use crate::{common::clients::corvidx_client, domain::entities::message};

pub fn on_init(corvidx: &DbConnection) {
	corvidx_client::subscribe_to_tables(&corvidx);
	message::subscribe_to_corvidx(&corvidx);
}
