use corvutils::ListFormat;
use teloxide::utils::command::CommandDescriptions;
use unicode_segmentation::UnicodeSegmentation;

pub trait CommandDescriptionsFormat {
	fn format_list(&self) -> String;
}

impl CommandDescriptionsFormat for CommandDescriptions<'_> {
	fn format_list(&self) -> String {
		let command_strings = self
			.to_string()
			.split('\n')
			.map(|s| process_command_with_emoji(s))
			.collect::<Vec<_>>();

		command_strings.format_list()
	}
}

fn process_command_with_emoji(command_string: &str) -> String {
	// Parse command string to extract command and description parts
	let parts: Vec<&str> = command_string.split(" — ").collect();
	if parts.len() != 2 {
		return command_string.to_string(); // Not in expected format, return as is
	}
	
	let command = parts[0];
	let description = parts[1];
	
	// Check if description starts with an emoji
	let graphemes: Vec<&str> = description.graphemes(true).collect();
	if !graphemes.is_empty() {
		let first_grapheme = graphemes[0];
		// Basic check if first grapheme might be an emoji (not perfect but reasonable)
		if is_likely_emoji(first_grapheme) {
			// Extract emoji and put it at the beginning of the command
			let emoji = first_grapheme;
			let description_without_emoji = &description[emoji.len()..].trim();
			return format!("{} {} — {}", emoji, command, description_without_emoji);
		}
	}
	
	// No emoji found or not at the start, return original format
	command_string.to_string()
}

fn is_likely_emoji(s: &str) -> bool {
	// This is a simplified check that looks for common emoji characteristics
	// Emojis are typically non-ascii and often in certain unicode ranges
	
	if s.chars().count() == 1 {
		// Single character check
		let c = s.chars().next().unwrap();
		// Common emoji unicode ranges
		return (c >= '\u{1F300}' && c <= '\u{1F6FF}') || // Misc Symbols and Pictographs
			   (c >= '\u{1F900}' && c <= '\u{1F9FF}') || // Supplemental Symbols and Pictographs
			   (c >= '\u{2600}' && c <= '\u{26FF}') ||   // Misc Symbols
			   (c >= '\u{2700}' && c <= '\u{27BF}') ||   // Dingbats
			   (c >= '\u{FE00}' && c <= '\u{FE0F}');     // Variation Selectors
	} else {
		// For multi-codepoint emojis, check if any character is likely an emoji component
		return s.chars().any(|c| 
			(c >= '\u{1F300}' && c <= '\u{1F6FF}') || 
			(c >= '\u{1F900}' && c <= '\u{1F9FF}') || 
			(c >= '\u{2600}' && c <= '\u{26FF}') || 
			(c >= '\u{2700}' && c <= '\u{27BF}') || 
			(c >= '\u{FE00}' && c <= '\u{FE0F}') ||
			// ZWJ (Zero Width Joiner) often used in complex emoji sequences
			c == '\u{200D}'
		);
	}
}
