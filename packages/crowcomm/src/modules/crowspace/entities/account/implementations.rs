use crate::crowspace::PublicProfileName;

impl ToString for PublicProfileName {
	fn to_string(&self) -> String {
		if let Some(name_extension) = &self.name_extension {
			format!("{} {}", self.short_name, name_extension)
		} else {
			self.short_name.clone()
		}
	}
}
