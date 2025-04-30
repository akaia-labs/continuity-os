use crowcomm::crowspace;

pub fn identifier(account: &crowspace::Account) -> String {
	account
		.callsign
		.clone()
		.unwrap_or_else(|| account.identity.to_hex().to_string())
}
