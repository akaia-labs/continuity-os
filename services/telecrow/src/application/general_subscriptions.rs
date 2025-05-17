use crowdcomm_sdk::corvidx::stdb::DbConnection;

use crate::{
	common::clients::corvidx_client,
	domain::entities::{corvidx_account, corvidx_message},
};

pub fn init(corvidx: &DbConnection) {
	corvidx_client::subscribe(&corvidx);
	corvidx_account::subscribe(&corvidx);
	corvidx_message::subscribe(&corvidx);
}
