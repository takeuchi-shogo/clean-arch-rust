

pub struct Config {
	pub enviroment: String,
	// pub db: DB,
	pub routing: Routing,
}

// pub struct DB {
// 	pub production: Production,
// 	pub test: Test,
// }

// pub struct Production {
// 	pub host: String,
// 	pub db_name: String,
// 	pub username: String,
// 	pub password: String,
// }

// pub struct Test {
// 	pub test_host: String,
// 	pub test_db_name: String,
// 	pub test_username: String,
// 	pub test_password: String,
// }

pub struct Routing {
	pub port: String,
}

impl Config {
	pub fn new_config() -> Config {
		Config {
			enviroment: String::from("development"),
			// db: DB {
			// 	production: Production {
			// 		host: String::from("localhost"),
			// 		db_name: String::from("rust_todo"),
			// 		username: String::from("user"),
			// 		password: String::from("pass"),
			// 	}, 
			// 	test: Test {
			// 		test_host: String::from("localhost"),
			// 		test_db_name: String::from("rust_todo_test"),
			// 		test_username: String::from("user"),
			// 		test_password: String::from("pass"),
			// 	},
			// },
			routing: Routing {
				port: String::from(":8080")
			},
		}
	}
}
