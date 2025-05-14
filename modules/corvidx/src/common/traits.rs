use spacetimedb::ReducerContext;

pub trait AsRecordResolver<RecordType> {
	fn resolve(&self, ctx: &ReducerContext) -> Result<RecordType, String>;
}
