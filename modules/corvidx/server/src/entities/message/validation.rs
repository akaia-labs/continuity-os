pub fn validate_message(text: String) -> Result<String, String> {
	if text.is_empty() {
		Err("Messages must not be empty".to_string())
	} else {
		Ok(text)
	}
}
