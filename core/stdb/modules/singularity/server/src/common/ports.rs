use spacetimedb::ReducerContext;

pub trait Resolvable {
	fn try_is_resolvable(&self, ctx: &ReducerContext) -> Result<(), String>;
}

pub trait RecordResolver<RecordType> {
	fn try_resolve(&self, ctx: &ReducerContext) -> Result<RecordType, String>;
}
