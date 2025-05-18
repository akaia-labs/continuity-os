use corvutils::ListFormat;
use teloxide_core::types::{Chat, ChatKind, PublicChatKind};

use crate::presentation::Summary;

impl Summary for Chat {
	fn summary(&self) -> String {
		let chat_type = match &self.kind {
			| ChatKind::Private(_) => "DM",

			| ChatKind::Public(props) => match &props.kind {
				| PublicChatKind::Channel(_) => "Channel",
				| PublicChatKind::Group => "Group",

				| PublicChatKind::Supergroup(supergroup_props) => supergroup_props
					.is_forum
					.then(|| "Forum")
					.unwrap_or("Supergroup"),
			},
		};

		vec![
			format!("Chat type: <code>{chat_type}</code>"),
			format!(
				"Chat title: {}",
				self.title()
					.map_or("not set".to_string(), |t| format!("<code>{t}</code>"))
			),
			format!("Chat ID: <code>{}</code>", self.id),
			format!(
				"Chat handle: {}",
				self.username()
					.map_or("not set".to_string(), |h| format!("<code>@{h}</code>"))
			),
		]
		.format_list()
	}
}
