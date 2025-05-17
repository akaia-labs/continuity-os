use spacetimedb::ReducerContext;

pub trait RecordResolution<RecordType> {
	fn resolve(&self, ctx: &ReducerContext) -> Result<RecordType, String>;
}
