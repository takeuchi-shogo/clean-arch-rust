
use async_trait::async_trait;
// use diesel::mysql::MysqlConnection;
use std::sync::Arc;
use crate::interfaces::gateways::database::db;
use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use crate::usecase::db_interface::DBInterface;

pub struct DBRepository {
	pub db: Arc<dyn db::DB + Sync + Send>,
}
impl DBRepository {
	// pub fn new(db: Arc<dyn db::DB>) -> Self {
	// 	Self {
	// 		db
	// 	}
	// }
}

#[async_trait]
impl DBInterface for DBRepository {
	fn connect(&self) -> &Pool<ConnectionManager<MysqlConnection>> {
		self.db.connect()
	}
}