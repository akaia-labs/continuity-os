use crowlink::clients::crownest;

/// Returns the user's name, or their identity if they have no name.
pub fn user_name_or_identity(user: &crownest::User) -> String {
	user.name
		.clone()
		.unwrap_or_else(|| user.identity.to_hex().to_string())
}
