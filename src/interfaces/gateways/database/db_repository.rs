
// use diesel::mysql::MysqlConnection;
use std::sync::Arc;
use crate::interfaces::gateways::database::db;
use crate::infrastructure::db::DBConn;
use crate::usecase::db_interface::DBInterface;

pub struct DBRepository {
	pub db: Arc<dyn db::DB>,
}

impl DBInterface for DBRepository {
	fn connect(&self) -> DBConn {
		self.db.connect()
	}
}