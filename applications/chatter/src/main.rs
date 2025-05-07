pub mod common;
pub mod entities;
pub mod features;

use crowcomm::crowd_core::{
	self, LocalAccountTableAccess, MessageTableAccess, send_message, set_account_callsign,
};
use entities::{foreign_account, message};
use spacetimedb_sdk::{Table, TableWithPrimaryKey};

use crate::{common::clients::crowd_core_client, entities::local_account, features::repl};

fn register_callbacks(ctx: &crowd_core::DbConnection) {
	ctx.db
		.local_account()
		.on_insert(local_account::on_account_inserted);

	ctx.db
		.local_account()
		.on_update(local_account::on_account_updated);

	ctx.db.message().on_insert(message::on_message_inserted);

	ctx.reducers
		.on_set_account_callsign(local_account::on_callsign_set);

	ctx.reducers.on_send_message(message::on_message_sent);
}

fn main() {
	let _ = dotenvy::dotenv();

	// Connect to the database
	let ctx = crowd_core_client::connect_to_db();

	register_callbacks(&ctx);
	crowd_core_client::subscribe_to_tables(&ctx);
	foreign_account::subscribe(&ctx);
	ctx.run_threaded();

	repl::start(&ctx);
}
