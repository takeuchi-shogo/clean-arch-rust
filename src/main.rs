#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

// use infrastructure::routing;

#[macro_use] extern crate rocket;

mod domain;
mod infrastructure;
mod interfaces;
mod usecase;

fn main() {
    let cfg = infrastructure::config::AppConfig::new_config();

    let routing = infrastructure::routing::Routing::new_routing(&cfg);
    routing.run(routing.port);
}