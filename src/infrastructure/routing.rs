// #![feature(proc_macro_hygiene, decl_macro)]

// use std::sync::Arc;
use rocket;

use super::{config::AppConfig, db::DB};
use crate::interfaces::controllers::product;
use crate::infrastructure::db;
use rocket::config::{ Config, Environment };

// #[derive(Debug)]
pub struct Routing<'a> {
	pub db: &'a DB,
	pub port: u16,
}

impl Routing<'_> {
	pub fn new_routing<'a>(cfg: &'a AppConfig, db: &'a DB) -> Routing<'a> {
		Routing {
			db,
			port: cfg.routing.port.to_owned(),
		}
	}

	pub fn run(&self, port: u16) {
		let config = Config::build(Environment::Staging)
			// .address("127.0.0.1")
			.port(port)
			.unwrap();

		let routing =  rocket::custom(config);

		routing
			.manage(db::init_db())
			// .mount("/", routes![name])
			.mount("/api",
		 routes![
				index,
				name,
				// get_users,
				])
			.launch();
	}

	pub fn get_user(&self) -> &'static str {
		let users_controller = product::users_controller::new_users_controller(self::db);
		users_controller.get_user()
	}
}

#[get("/<name>")]
fn name(name: String) -> String {
	format!("Hello, {}", name)
}

#[get("/hello")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/users")]
fn get_users() -> &'static str {
	// let users_controller = product::users_controller::new_users_controller(db);
	// users_controller.get_user()
	&Routing::get_user()
}