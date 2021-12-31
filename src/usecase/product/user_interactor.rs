
use std::sync::Arc;

use crate::usecase::user_repository::UserInterface;

pub struct UserInteractor {
// pub struct UserInteractor {
	pub user: Arc<dyn UserInterface + Sync + Send>,
	// pub word: String,
}

// #[async_trait]
impl UserInteractor {

	pub fn get_users(&self, w: &'static str) -> &'static str {
		self.user.find_by_id(w)
		// ここで一旦、変数word　に"Hello World!" を入れたい
	}
}