use std::sync::Arc;

use crowdcomm_sdk::{
	corvidx::stdb::DbConnection,
	integrations::{commands::AlrActionResolution, dtos::ActionResolutionCommand},
};

use crate::BotInstanceType;

pub fn handle_request_callback(
	ctx: Arc<DbConnection>, bot: BotInstanceType,
	command: ActionResolutionCommand<AlrActionResolution>,
) {
	match command.payload {
		| AlrActionResolution::Accept(id) => {
			// let alr = corvidx_conn.db.account_link_request().id().find(&
			// id) 	.ok_or(format!("Account link request {id} does
			// not exist."));

			println!("Account link request {id} has been accepted.");
		},

		| AlrActionResolution::Reject(id) => {
			println!("Account link request {id} has been rejected.");
		},
	}
}
