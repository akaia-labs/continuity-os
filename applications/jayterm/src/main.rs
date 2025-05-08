pub mod common;
pub mod entities;
pub mod features;

use crowdcomm::corvidx::{
	self, LocalAccountTableAccess, MessageTableAccess, send_message, set_account_callsign,
};
use entities::{foreign_account, message};
use spacetimedb_sdk::{Table, TableWithPrimaryKey};

use crate::{common::clients::corvidx_client, entities::local_account, features::repl};

fn register_callbacks(corvidx: &corvidx::DbConnection) {
	corvidx
		.db
		.local_account()
		.on_insert(local_account::on_account_inserted);

	corvidx
		.db
		.local_account()
		.on_update(local_account::on_account_updated);

	corvidx.db.message().on_insert(message::on_message_inserted);

	corvidx
		.reducers
		.on_set_account_callsign(local_account::on_callsign_set);

	corvidx.reducers.on_send_message(message::on_message_sent);
}

fn main() {
	let _ = dotenvy::dotenv();

	// Connect to the database
	let corvidx = corvidx_client::connect_to_db();

	register_callbacks(&corvidx);
	corvidx_client::subscribe_to_tables(&corvidx);
	foreign_account::subscribe(&corvidx);
	corvidx.run_threaded();

	repl::start(&corvidx);
}
