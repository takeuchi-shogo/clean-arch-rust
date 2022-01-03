
use diesel;
use diesel::mysql::MysqlConnection;
use diesel::r2d2::ConnectionManager;
// use diesel;

use crate::interfaces::gateways::database;

use super::config::AppConfig;

pub struct DB {
	pub host: String,
	pub db_name: String,
	pub username: String,
	pub password: String,
	// pub connection: DBConn,
}

pub type Pool<T> = r2d2::Pool<ConnectionManager<T>>;
pub type MysqlPool = Pool<MysqlConnection>;

pub type DBConn = MysqlPool;

impl DB {
	pub fn new_database(cfg: &AppConfig) -> DB {
		DB {
			// &で借用している状態
			// 所有権まで移動できない
			host: cfg.db.production.host.clone(),
			db_name: cfg.db.production.db_name.clone(),
			username: cfg.db.production.username.clone(),
			password: cfg.db.production.password.clone(),
			// connection: sql_pool(),
		}
	}

	// pub fn new_DB(&self) -> DB {
	// 	// Go ではここでDB_URLの生成とDBとの接続確認を行なっていた。
	// 	let database_url = "mysql://root:@localhost/diesel_test";
	// 	let manager = ConnectionManager::<MysqlConnection>::new(&database_url);
	// 	Pool::new(manager).expect(&format!("Error connection to {}", database_url));
	// 	&self.connection = 
	// }
}

impl database::db::DB for DB {
	// fn connect(&self) -> DBConn {
	// 	DB::connection
	// }
	fn connect(&self) -> DBConn {
		let database_url = "mysql://root:@localhost/diesel_test";
		let manager = ConnectionManager::<MysqlConnection>::new(database_url);
		Pool::builder()
			.build(manager)
			.expect(&format!("Error connection to {}", database_url))
	}
}
