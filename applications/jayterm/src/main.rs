pub mod common;
pub mod entities;
pub mod features;

use dotenvy::dotenv;
use entities::{external_actor, message};
use features::account_linking;

use crate::{common::clients::singularity_client, entities::account, features::repl};

fn main() {
	let _ = dotenv();

	let singularity = singularity_client::connect_to_db();

	singularity_client::subscribe_to_tables(&singularity);
	account::subscribe(&singularity);
	external_actor::subscribe(&singularity);
	message::subscribe(&singularity);
	account_linking::subscribe(&singularity);
	singularity.run_threaded();

	repl::start(&singularity);
}
