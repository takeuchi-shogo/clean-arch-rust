
use diesel;
use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool};

use async_trait::async_trait;

use crate::interfaces::gateways::database;

use super::config::AppConfig;

// #[derive(Debug)]
pub struct DB {
	pub host: String,
	pub db_name: String,
	pub username: String,
	pub password: String,
	pub connection: Pool<ConnectionManager<MysqlConnection>>,
}

// pub type Pool<T> = Pool<ConnectionManager<T>>;
// pub type MysqlPool = Pool<MysqlConnection>;

// pub type DBConn = MysqlPool;

impl DB {
	pub fn new_database(cfg: &AppConfig) -> DB {
		DB {
			// &で借用している状態
			// 所有権まで移動できない
			host: cfg.db.production.host.clone(),
			db_name: cfg.db.production.db_name.clone(),
			username: cfg.db.production.username.clone(),
			password: cfg.db.production.password.clone(),
			connection: DB::new(),
		}
	}

	pub fn new() -> Pool<ConnectionManager<MysqlConnection>> {
		// Go ではここでDB_URLの生成とDBとの接続確認を行なっていた。
		let database_url = "mysql://root:@localhost/diesel_test";
		let manager = ConnectionManager::<MysqlConnection>::new(&database_url.to_owned());
		let connection = Pool::new(manager).expect(&format!("Error connection to {}", database_url));
		connection
	}

	// fn connect(&self) -> Pool<ConnectionManager<MysqlConnection>> {
	// 	self.connection
	// }
}

#[async_trait]
impl database::db::DB for DB {
	// fn connect(&self) -> DBConn {
	// 	DB::connection
	// }
	fn connect(&self) -> &Pool<ConnectionManager<MysqlConnection>> {
		&self.connection
	}
}

pub fn init_db() -> Pool<ConnectionManager<MysqlConnection>> {
	let url = "mysql://root:@localhost/diesel_test";
	let manager = ConnectionManager::<MysqlConnection>::new(url);
	Pool::builder()
		.build(manager)
		.expect(&format!("Error connection to {}", &url))
}