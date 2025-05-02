use std::fmt;

/// Enum to refer to token *types* semantically
#[derive(Copy, Clone)]
pub enum SocialNameTokenKind {
	ShortName,
	NameExtension,
}

/// Individual name fragments with semantic meaning
// #[derive(Serialize, Deserialize)]
pub enum SocialNameToken {
	ShortName(String),
	NameExtension(String),
}

impl fmt::Display for SocialNameToken {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			| SocialNameToken::ShortName(s) => write!(f, "{}", s),
			| SocialNameToken::NameExtension(s) => write!(f, " {}", s),
		}
	}
}

/// Logical grouping of name tokens
// #[derive(Serialize, Deserialize)]
pub struct SocialName {
	pub tokens: Vec<SocialNameToken>,
}

impl fmt::Display for SocialName {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		for token in &self.tokens {
			write!(f, "{}", token)?;
		}

		Ok(())
	}
}

impl SocialName {
	pub fn get_token(&self, requested_kind: SocialNameTokenKind) -> Option<&String> {
		self.tokens
			.iter()
			.find_map(|token| match (requested_kind, token) {
				| (SocialNameTokenKind::ShortName, SocialNameToken::ShortName(value)) => {
					Some(value)
				},

				| (SocialNameTokenKind::NameExtension, SocialNameToken::NameExtension(value)) => {
					Some(value)
				},

				| (SocialNameTokenKind::ShortName, SocialNameToken::NameExtension(_)) => None,
				| (SocialNameTokenKind::NameExtension, SocialNameToken::ShortName(_)) => None,
			})
	}
}

// #[derive(Serialize, Deserialize)]
pub struct SocialProfile {
	pub name: SocialName,
}
