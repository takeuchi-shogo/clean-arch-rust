
// use diesel::mysql::MysqlConnection;

use crate::infrastructure::db::DBConn;
pub trait DB {
	fn connect(&self) -> DBConn;
}