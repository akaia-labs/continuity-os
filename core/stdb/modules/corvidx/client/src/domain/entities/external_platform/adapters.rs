use std::str::FromStr;

use super::SupportedExternalActorOrigin;
use crate::common::stdb::ExternalActorOrigin;

impl ExternalActorOrigin {
	pub fn into_supported(&self) -> SupportedExternalActorOrigin {
		(*self).into()
	}
}

impl From<SupportedExternalActorOrigin> for ExternalActorOrigin {
	fn from(name_reference: SupportedExternalActorOrigin) -> Self {
		match name_reference {
			| SupportedExternalActorOrigin::Telegram => ExternalActorOrigin::Telegram,
			| SupportedExternalActorOrigin::Unknown => ExternalActorOrigin::Unknown,
		}
	}
}

impl Into<SupportedExternalActorOrigin> for ExternalActorOrigin {
	fn into(self) -> SupportedExternalActorOrigin {
		match self {
			| ExternalActorOrigin::Telegram => SupportedExternalActorOrigin::Telegram,
			| ExternalActorOrigin::Unknown => SupportedExternalActorOrigin::Unknown,
		}
	}
}

impl FromStr for ExternalActorOrigin {
	type Err = &'static str;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(ExternalActorOrigin::from(
			match s.parse::<SupportedExternalActorOrigin>() {
				| Ok(platform_tag) => platform_tag,
				| Err(_) => SupportedExternalActorOrigin::Unknown,
			},
		))
	}
}
