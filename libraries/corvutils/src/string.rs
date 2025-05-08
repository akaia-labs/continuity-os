pub fn to_single_line(s: &str) -> String {
	// 1. Trim only indentation (e.g. via `unindent`)
	let dedented = unindent::unindent(s);

	// 2. Collapse all adjacent whitespace to single spaces
	let collapsed = whitespace_sifter::collapse(&dedented);

	// 3. Re-attach outer newlines
	format!("{}", collapsed.trim(),)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_to_single_line() {
		let external_identifier = "@username:example.com";
		let platform_name = "matrix";

		let formatted_example = to_single_line(format!(
			r#"
				Your profile has been updated to match the appearance of 
				{external_identifier} {platform_name} account.
			"#
		));

		let expected_output = "Your profile has been updated to match the appearance of \
		                       @username:example.com matrix account.";

		assert_eq!(formatted_example, expected_output);
	}
}
