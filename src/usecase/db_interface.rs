// use diesel::MysqlConnection;

use crate::infrastructure::db::DBConn;


pub trait DBInterface {
	fn connect(&self) -> DBConn;
}