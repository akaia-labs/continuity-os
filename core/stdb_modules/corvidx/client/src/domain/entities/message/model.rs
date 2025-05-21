use crate::common::stdb::NativeAccountLocalRole;

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

	pub fn by_account_role(role: Option<NativeAccountLocalRole>) -> Self {
		match role {
			| Some(known_role) => match known_role {
				| NativeAccountLocalRole::Admin | NativeAccountLocalRole::Interactor => {
					Self::Content
				},

				| NativeAccountLocalRole::Service => Self::System,
			},

			| _ => Self::Content,
		}
	}

	pub fn symbol_by_account_role(role: Option<NativeAccountLocalRole>) -> &'static str {
		Self::by_account_role(role).symbol()
	}
}
