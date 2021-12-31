
use std::sync::Arc;

use crate::usecase::user_interface::UserInterface;

pub struct UserInteractor {
	pub user: Arc<dyn UserInterface + Sync + Send>,
}

// #[async_trait]
impl UserInteractor {

	pub fn get_users(&self, w: &'static str) -> &'static str {
		self.user.find_by_id(w)
	}
}