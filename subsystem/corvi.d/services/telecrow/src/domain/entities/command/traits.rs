use corvutils::{ListFormat, SliceExtensions};
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
			.map(|s| process_with_emoji(s))
			.collect::<Vec<_>>();

		command_strings.format_list()
	}
}

fn process_with_emoji(command_string: &str) -> String {
	// Parse command string to extract command and description parts
	let parts: Vec<&str> = command_string.split(" — ").collect();

	if parts.len() != 2 {
		// Not in expected format, return as is
		return command_string.to_string();
	}

	let command = parts[0];
	let description = parts[1];

	let graphemes: Vec<&str> = description.graphemes(true).collect();

	// Check if description starts with an emoji
	if !graphemes.is_empty() {
		let first_grapheme = graphemes[0];

		// Extract emoji and put it at the beginning of the command
		if first_grapheme.is_likely_emoji() {
			let emoji = first_grapheme;
			let description_without_emoji = &description[emoji.len() ..].trim();

			return format!("{} {} — {}", emoji, command, description_without_emoji);
		}
	}

	// No emoji found at the start, return original format
	command_string.to_string()
}
