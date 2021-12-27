#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

#[macro_use] extern crate rocket;

mod infrastructure;

fn main() {
    let cfg = infrastructure::config::Config::new_config();

    let routing = infrastructure::routing::Routing::new_routing(&cfg);
    infrastructure::routing::Routing::run(&routing.port);
}