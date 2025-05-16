use crate::common::stdb::ForeignAccountReference;

pub trait ForeignAccountImport {
	/// Puts third-party account data into locally recognized format
	fn into_account_reference(&self) -> ForeignAccountReference;
}
