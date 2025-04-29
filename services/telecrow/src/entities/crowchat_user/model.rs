use crowtocol_rs::crowchat;

/// Returns the user's name, or their identity if they have no name.
pub fn name_or_identity(user: &crowchat::User) -> String {
	user.name
		.clone()
		.unwrap_or_else(|| user.identity.to_hex().to_string())
}

/**
 * TODO: Implement the core part of the following code in the crowchat db module itself
 */
use std::fmt;

/// Enum to refer to token *types* semantically
#[derive(Copy, Clone)]
pub enum UserIdentifierTokenKind {
	ShortName,
	NameExtension,
}

/// Individual name fragments with semantic meaning
// #[derive(Serialize, Deserialize)]
pub enum UserIdentifierToken {
	ShortName(String),
	NameExtension(String),
}

impl fmt::Display for UserIdentifierToken {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			| UserIdentifierToken::ShortName(s) => write!(f, "{}", s),
			| UserIdentifierToken::NameExtension(s) => write!(f, " {}", s),
		}
	}
}

/// Logical grouping of name tokens
// #[derive(Serialize, Deserialize)]
pub struct UserIdentifier {
	pub tokens: Vec<UserIdentifierToken>,
}

impl fmt::Display for UserIdentifier {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		for token in &self.tokens {
			write!(f, "{}", token)?;
		}

		Ok(())
	}
}

impl UserIdentifier {
	pub fn get_token(&self, kind: UserIdentifierTokenKind) -> Option<&String> {
		self.tokens.iter().find_map(|token| match (kind, token) {
			| (UserIdentifierTokenKind::ShortName, UserIdentifierToken::ShortName(s)) => Some(s),
			| (UserIdentifierTokenKind::ShortName, UserIdentifierToken::NameExtension(_)) => None,

			| (UserIdentifierTokenKind::NameExtension, UserIdentifierToken::NameExtension(s)) => {
				Some(s)
			},
			| (UserIdentifierTokenKind::NameExtension, UserIdentifierToken::ShortName(_)) => None,
		})
	}
}

/// Metadata — could include avatar, status, etc.
// #[derive(Serialize, Deserialize)]
pub struct SenderMetadata {
	pub name: UserIdentifier,
}
