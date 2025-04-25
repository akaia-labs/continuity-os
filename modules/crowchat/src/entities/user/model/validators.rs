/// Takes a name and checks if it's acceptable as a user's name.
pub fn validate_name(name: String) -> Result<String, String> {
	if name.is_empty() {
		Err("Names must not be empty".to_string())
	} else {
		Ok(name)
	}
}
