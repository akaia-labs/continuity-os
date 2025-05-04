use spacetimedb_sdk::credentials;

pub fn credential_store() -> credentials::File {
	credentials::File::new("telecrow.credentials")
}
