
use async_trait::async_trait;

use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool};

#[async_trait]
pub trait DBInterface {
	fn connect(&self) -> &Pool<ConnectionManager<MysqlConnection>>;
}