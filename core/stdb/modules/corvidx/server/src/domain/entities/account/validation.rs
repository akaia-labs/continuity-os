pub fn validate_callsign(callsign: String) -> Result<String, String> {
	if callsign.is_empty() {
		Err("Callsign must not be empty".to_string())
	} else {
		Ok(callsign)
	}
}
