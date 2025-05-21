use spacetimedb::ReducerContext;

pub trait RecordResolution<RecordType> {
	fn try_resolve(&self, ctx: &ReducerContext) -> Result<RecordType, String>;
}
