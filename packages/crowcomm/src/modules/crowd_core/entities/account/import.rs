use crate::crowd_core::ForeignAccount;

pub trait ForeignAccountImport {
	fn into_account(&self) -> ForeignAccount;
}
