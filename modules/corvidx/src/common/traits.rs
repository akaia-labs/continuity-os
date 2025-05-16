use spacetimedb::ReducerContext;

pub trait RecordResolver<RecordType> {
	fn resolve(&self, ctx: &ReducerContext) -> Result<RecordType, String>;
}
