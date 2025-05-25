use crate::common::stdb::AccountRole;

pub enum MessageType {
	System,
	Content,
	Unknown,
}

impl MessageType {
	pub fn symbol(&self) -> &'static str {
		match self {
			| Self::System => "ℹ️",
			| Self::Content => "💬",
			| Self::Unknown => "❓",
		}
	}

	pub fn by_account_role(role: Option<AccountRole>) -> Self {
		match role {
			| Some(known_role) => match known_role {
				| AccountRole::Admin | AccountRole::Interactor => {
					Self::Content
				},

				| AccountRole::Service => Self::System,
			},

			| _ => Self::Content,
		}
	}

	pub fn symbol_by_account_role(role: Option<AccountRole>) -> &'static str {
		Self::by_account_role(role).symbol()
	}
}
