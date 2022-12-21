
use async_trait::async_trait;

use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool};

// use crate::infrastructure::db::DBConn;
#[async_trait]
pub trait DB {
	fn connect(&self) -> &Pool<ConnectionManager<MysqlConnection>>;
}