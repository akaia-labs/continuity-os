mod authentication;
mod subscriptions;

use crowdcomm_sdk::{
	configuration::corvid_subsystem_config::{self, CorvidSubsystemConfig},
	corvidx::stdb::DbConnection,
};
use spacetimedb_sdk::DbContext;

/// Loads credentials from a file and connects to the database.
pub fn connect() -> DbConnection {
	let CorvidSubsystemConfig {
		module_host,
		components,
	} = corvid_subsystem_config::get();

	DbConnection::builder()
			.on_connect(subscriptions::on_connected)
			.on_connect_error(subscriptions::on_connect_error)
			.on_disconnect(subscriptions::on_disconnected)
			// If the account has previously connected, we'll have saved a token in the `on_connect` callback.
			// In that case, we'll load it and pass it to `with_token`,
			// so we can re-authenticate as the same `Identity`.
			.with_token(authentication::credential_store().load()
				.expect("Error loading credentials")
			)
			.with_module_name(components.corvidx.db_name)
			.with_uri(module_host)
			.build()
			.expect("Failed to connect")
}

/// Registers subscriptions to tables.
pub fn subscribe(corvidx: &DbConnection) {
	corvidx
		.subscription_builder()
		.on_applied(subscriptions::on_sub_applied)
		.on_error(subscriptions::on_sub_error)
		// Facilitating creation of a local partial replica of the database.
		.subscribe([
			"SELECT * FROM account_link_request",
			"SELECT * FROM account_profile",
			"SELECT * FROM foreign_account",
			"SELECT * FROM message",
			// "SELECT * FROM message_channel",
			"SELECT * FROM native_account",
		]);
}
