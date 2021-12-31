use crate::usecase::user_repository::UserInterface;


pub struct UserRepository {}

impl UserRepository {
	pub fn new() -> Self {
		Self { /* fields */ }
	}
}

impl UserInterface for UserRepository {
	fn find_by_id(&self,
		w: &'static str) -> &'static str {
		let wg = w;
		wg
	}
}