mod authentication;
mod subscriptions;

use core::panic;

use crowcomm::{crowd_core::DbConnection, get_env_config};
use spacetimedb_sdk::DbContext;

/// Loads credentials from a file and connects to the database.
pub fn connect() -> DbConnection {
	if let Some(env_config) = get_env_config() {
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
			.with_module_name(env_config.modules.crowspace.name)
			.with_uri(env_config.host)
			.build()
			.expect("Failed to connect")
	} else {
		panic!(
			"❌ Missing environment variables! Check your .env file and .env.example reference."
		);
	}
}

/// Registers subscriptions to tables.
pub fn subscribe(crowspace_ctx: &DbConnection) {
	crowspace_ctx
		.subscription_builder()
		.on_applied(subscriptions::on_sub_applied)
		.on_error(subscriptions::on_sub_error)
		// Facilitating creation of a local partial replica of the database.
		.subscribe([
			"SELECT * FROM account",
			"SELECT * FROM external_account",
			"SELECT * FROM message",
			"SELECT * FROM public_profile"
		]);
}
