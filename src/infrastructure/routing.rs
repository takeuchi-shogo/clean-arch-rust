
// #[macro_use] extern crate rocket;
use rocket;

// use rocket::fairing::AdHoc;

use super::config::AppConfig;
use crate::interfaces::controllers::product;
use rocket::config::{ Config, Environment };
// use rocket::fairing::AdHoc;
// use rocket::serde::Deserialize;


#[derive(Debug)]
pub struct Routing {
	pub port: u16,
}

impl Routing {
	pub fn new_routing(cfg: &AppConfig) -> Routing {
		Routing {
			port: cfg.routing.port.to_owned(),
		}
	}

	pub fn run(&self, port: u16) {
		println!(
			"{}",
			port,
		);
		let config = Config::build(Environment::Staging)
			// .address("127.0.0.1")
			.port(port)
			.unwrap();
		let routing =  rocket::custom(config);

		// let usersController = product::users_controller::new_users_controller();
		// usersController.get_users();
		// println!("{}", usersController);
		routing.mount("/api",
		 routes![
				index,
				get_users,
				])
			.launch();
	}
}

#[get("/hello")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/users")]
pub fn get_users() -> &'static str {
	let users_controller = product::users_controller::new_users_controller();
	users_controller.get_user()
}