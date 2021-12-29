
use crate::usecase::product::user_interactor::UserInteractor;

#[derive(Default)]
pub struct UsersController {
	pub interactor: UserInteractor,
}

pub fn new_users_controller() -> UsersController {
	UsersController {
		interactor: UserInteractor{
			word: "a".to_string(),
		},
	}
}

impl UsersController {

	// #[get("/users")]
	pub fn get_user(&self) -> &'static str {
		let w = "Hello Rust! Very hard";
		// let res = interactor::user_interactor::get(w);
		self.interactor.get(w)
	}
}