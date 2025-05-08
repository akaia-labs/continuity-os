pub mod common;
pub mod entities;
pub mod features;

use dotenvy::dotenv;
use entities::{foreign_account, message};
use features::account_linking;

use crate::{common::clients::corvidx_client, entities::local_account, features::repl};

fn main() {
	let _ = dotenv();

	let corvidx = corvidx_client::connect_to_db();

	corvidx_client::subscribe_to_tables(&corvidx);
	local_account::subscribe(&corvidx);
	foreign_account::subscribe(&corvidx);
	message::subscribe(&corvidx);
	account_linking::subscribe(&corvidx);
	corvidx.run_threaded();

	repl::start(&corvidx);
}
