use crate::{
	crowd_core::{ForeignAccount, account::ForeignAccountImport},
	telegram,
};

impl ForeignAccountImport for telegram::User {
	fn into_account(&self) -> ForeignAccount {
		ForeignAccount {
			id:         (),
			owner_id:   (),
			profile_id: (),
		}
	}
}
