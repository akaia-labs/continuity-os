use crate::crowd_core::AccountProfileMetadata;

pub trait ProfileImport {
	/// Puts third-party profile into locally recognized format
	fn into_profile_metadata(&self) -> AccountProfileMetadata;
}
