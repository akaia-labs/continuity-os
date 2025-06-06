pub trait SliceExtensions {
	/// Checks if a string slice likely contains an emoji
	fn is_likely_emoji(&self) -> bool;
}

impl SliceExtensions for str {
	/// Attempts to recognize common emoji patterns.
	fn is_likely_emoji(&self) -> bool {
		if self.chars().count() == 1 {
			let c = self.chars().next().unwrap();

			// Common emoji unicode ranges
			return
				// Misc Symbols and Pictographs
				(c >= '\u{1F300}' && c <= '\u{1F6FF}') ||

				// Supplemental Symbols and Pictographs
				(c >= '\u{1F900}' && c <= '\u{1F9FF}') ||

				// Misc Symbols
				(c >= '\u{2600}' && c <= '\u{26FF}') ||

				// Dingbats
				(c >= '\u{2700}' && c <= '\u{27BF}') ||

				// Variation Selectors
				(c >= '\u{FE00}' && c <= '\u{FE0F}') ||

				// Miscellaneous Symbols and Arrows
				(c >= '\u{2B00}' && c <= '\u{2BFF}') ||

				// Mahjong Tiles
				(c >= '\u{1F000}' && c <= '\u{1F02F}') ||

				// Regional Indicator Symbols (flags)
				(c >= '\u{1F1E0}' && c <= '\u{1F1FF}') ||

				// Explicit check for star
				(c == '⭐');
		} else {
			// Special case for flag emojis (typically two Regional Indicator Symbols)
			if self.chars().count() == 2
				&& self.chars().all(|c| c >= '\u{1F1E0}' && c <= '\u{1F1FF}')
			{
				return true;
			}

			// For multi-codepoint emojis, check if any character is likely an emoji
			// component
			return self.chars().any(|c| {
				(c >= '\u{1F300}' && c <= '\u{1F6FF}') ||
				(c >= '\u{1F900}' && c <= '\u{1F9FF}') ||
				(c >= '\u{2600}' && c <= '\u{26FF}') ||
				(c >= '\u{2700}' && c <= '\u{27BF}') ||
				(c >= '\u{FE00}' && c <= '\u{FE0F}') ||
				(c >= '\u{2B00}' && c <= '\u{2BFF}') ||
				(c >= '\u{1F000}' && c <= '\u{1F02F}') ||

				// Regional Indicator Symbols (flags)
				(c >= '\u{1F1E0}' && c <= '\u{1F1FF}') ||

				// ZWJ (Zero Width Joiner) often used in complex emoji sequences
				c == '\u{200D}' ||

				c == '⭐'
			});
		}
	}
}

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
		let origin = "matrix";

		let formatted_example = format!(
			r#"
				Your profile has been updated to match the appearance of 
				{external_identifier} {origin} account.
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

	#[test]
	fn test_is_likely_emoji() {
		// Test single-character emojis
		assert!("😀".is_likely_emoji());
		assert!("🌍".is_likely_emoji());
		assert!("⭐".is_likely_emoji());

		// Test multi-character emojis
		assert!("👨‍💻".is_likely_emoji());
		assert!("🇺🇸".is_likely_emoji());

		// Test non-emoji strings
		assert!(!"Hello".is_likely_emoji());
		assert!(!"12345".is_likely_emoji());
		assert!(!"!@#$%".is_likely_emoji());
	}
}
