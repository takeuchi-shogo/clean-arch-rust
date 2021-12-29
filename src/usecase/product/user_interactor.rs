
#[derive(Default)]
pub struct UserInteractor {
	// user: String,
	pub word: String,
}

impl UserInteractor {
	pub fn get(&self, w: &'static str) -> &'static str {
		// ここで一旦、変数word　に"Hello World!" を入れたい
		w
		
	}
}