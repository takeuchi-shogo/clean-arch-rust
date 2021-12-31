
// #![feature(trace_macros)]
// #[async_trait]
pub trait UserInterface {
	fn find_by_id(&self, w: &'static str) -> &'static str;
}