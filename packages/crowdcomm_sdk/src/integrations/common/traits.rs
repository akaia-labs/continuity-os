use corvidx_client::common::stdb::{AccountProfileMetadata, ForeignAccountReference};

pub trait ForeignAccountImport {
	/// Puts third-party account data into locally recognized format
	fn into_account_reference(&self) -> ForeignAccountReference;
}

pub trait ProfileImport {
	/// Puts third-party profile into locally recognized format
	fn into_profile_metadata(&self) -> AccountProfileMetadata;
}
