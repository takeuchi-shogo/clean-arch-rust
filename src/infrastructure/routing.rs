
// #[macro_use] extern crate rocket;
use rocket;

// use rocket::fairing::AdHoc;

use super::config::Config;
// use rocket::{State, Config};
// use rocket::fairing::AdHoc;
// use rocket::serde::Deserialize;


#[derive(Debug)]
pub struct Routing {
	pub port: String,
}

impl Routing {
	pub fn new_routing(cfg: &Config) -> Routing {
		Routing {
			port: cfg.routing.port.to_owned(),
		}
	}

	pub fn run(port: &String) {
		// rocket.costom(&Config)
		println!(
			"{}",
			port,
		);
		// rocket::build()
		// 	.mount("/hello", route![index])
		// 	.attach(AdHoc::config::<Routing>())
		rocket::ignite().mount("/", routes![index]).launch();
	}
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}