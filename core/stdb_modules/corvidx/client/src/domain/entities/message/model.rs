use crate::common::stdb::NativeAccountLocalRole;

pub enum MessageType {
	System,
	Content,
	Unknown,
}

impl MessageType {
	pub fn symbol(&self) -> &'static str {
		match self {
			| MessageType::System => "ℹ️",
			| MessageType::Content => "💬",
			| MessageType::Unknown => "❓",
		}
	}

	pub fn by_account_role(role: Option<NativeAccountLocalRole>) -> Self {
		match role {
			| Some(known_role) => match known_role {
				| NativeAccountLocalRole::Admin | NativeAccountLocalRole::Interactor => {
					MessageType::Content
				},

				| NativeAccountLocalRole::Service => MessageType::System,
			},

			| _ => MessageType::Unknown,
		}
	}

	pub fn symbol_by_account_role(role: Option<NativeAccountLocalRole>) -> &'static str {
		Self::by_account_role(role).symbol()
	}
}
