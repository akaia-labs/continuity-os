use crowtocol_rs::crowchat;

/// Returns the user's name, or their identity if they have no name.
pub fn name_or_identity(user: &crowchat::User) -> String {
	user.name
		.clone()
		.unwrap_or_else(|| user.identity.to_hex().to_string())
}
