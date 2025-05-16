/// A local alternative to [`ToString`] for imported types
/// like [`teloxide::types::Chat`].
pub trait Summarizable {
	fn summary(&self) -> String;
}
