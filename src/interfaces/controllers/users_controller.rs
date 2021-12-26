
mod interactor;

pub struct UsersController {
	pub interactor: interactor::UserInteractor,
}

impl UsersController {
	pub fn get() {
		let w = "Hello World!";
		// let res = interactor::user_interactor::get(w);
		interactor::user_interactor::get(w);
	}
}