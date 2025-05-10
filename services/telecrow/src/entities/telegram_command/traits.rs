use corvutils::ListFormat;
use teloxide::utils::command::CommandDescriptions;

pub trait CommandDescriptionsFormat {
	fn format_list(&self) -> String;
}

impl CommandDescriptionsFormat for CommandDescriptions<'_> {
	fn format_list(&self) -> String {
		let command_strings = self
			.to_string()
			.split('\n')
			.map(|s| s.to_string())
			.collect::<Vec<_>>();

		command_strings.format_list()
	}
}
