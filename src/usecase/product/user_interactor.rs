
use std::sync::Arc;

use crate::usecase::{user_interface::UserInterface, db_interface::DBInterface};

pub struct UserInteractor {
	pub db: Arc<dyn DBInterface + Send + Sync>,
	pub user: Arc<dyn UserInterface + Sync + Send>,
}

// #[async_trait]
impl UserInteractor {

	pub fn get_users(&self, w: &'static str) -> &'static str {
		let _db = self.db.connect();
		self.user.find_by_id(w)
	}
}