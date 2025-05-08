pub trait StringExtensions {
	/// Converts a multi-line string to a single-line string.
	fn squash_whitespace(self) -> String;
}

impl StringExtensions for String {
	fn squash_whitespace(self) -> String {
		self.split_whitespace().collect::<Vec<_>>().join(" ")
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_squash_whitespace() {
		let external_identifier = "@username:example.com";
		let platform_name = "matrix";

		let formatted_example = format!(
			r#"
				Your profile has been updated to match the appearance of 
				{external_identifier} {platform_name} account.
			"#
		)
		.squash_whitespace();

		let expected_output = "Your profile has been updated to match the appearance of \
		                       @username:example.com matrix account.";

		dbg!(formatted_example.clone());
		assert_eq!(formatted_example, expected_output);
	}
}
