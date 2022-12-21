
use std::sync::Arc;

use crate::interfaces::gateways::database;
use crate::interfaces::gateways::database::db::DB;
use crate::interfaces::gateways::database::db_repository::DBRepository;
use crate::usecase::product::user_interactor::UserInteractor;

// #[derive(Default)]
pub struct UsersController {
	pub interactor: UserInteractor,
}

pub fn new_users_controller(db: Arc<dyn DB + Send + Sync>) -> UsersController {
	UsersController {
		interactor: UserInteractor{
			db: Arc::new(DBRepository{ db: db }),
			user: Arc::new(database::user_repository::UserRepository{}),
			// word: "a".to_string(),
		},
	}
}

impl UsersController {

	// #[get("/users")]
	pub fn get_user(&self) -> &'static str {
		let w = "Hello Rust! Very hard";
		// let res = interactor::user_interactor::get(w);
		self.interactor.get_users(w)
	}
}