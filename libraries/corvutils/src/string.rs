pub trait StringExtensions {
	/// Converts a multi-line string to a single-line string.
	fn squash_whitespace(self) -> String;

	/// Wraps a string in newline characters.
	fn padded(self) -> String;
}

impl StringExtensions for String {
	fn squash_whitespace(self) -> String {
		self.split_whitespace().collect::<Vec<_>>().join(" ")
	}

	fn padded(self) -> String {
		format!("\n{}\n", self)
	}
}

pub trait ListFormat {
	fn format_list(self) -> String;
}

impl ListFormat for Vec<String> {
	fn format_list(self) -> String {
		self.iter()
			.map(|s| s.clone().padded())
			.collect::<Vec<_>>()
			.join("")
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_squash_whitespace() {
		let external_identifier = "@username:example.com";
		let platform_tag = "matrix";

		let formatted_example = format!(
			r#"
				Your profile has been updated to match the appearance of 
				{external_identifier} {platform_tag} account.
			"#
		)
		.squash_whitespace();

		let expected_output = "Your profile has been updated to match the appearance of \
		                       @username:example.com matrix account.";

		dbg!(formatted_example.clone());
		assert_eq!(formatted_example, expected_output);
	}

	#[test]
	fn test_padded() {
		let formatted_example = "test".to_string().padded();
		let expected_output = "\ntest\n";

		dbg!(formatted_example.clone());
		assert_eq!(formatted_example, expected_output);
	}

	#[test]
	fn test_format_list() {
		let items = vec![
			"First item".to_string(),
			"Second item".to_string(),
			"Third item".to_string(),
		];

		let formatted_example = items.format_list();
		let expected_output = "\nFirst item\n\nSecond item\n\nThird item\n";

		dbg!(formatted_example.clone());
		assert_eq!(formatted_example, expected_output);
	}
}
