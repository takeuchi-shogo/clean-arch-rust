#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

#[macro_use] extern crate rocket;

mod domain;
mod infrastructure;
mod interfaces;
mod usecase;

fn main() {
    let cfg = infrastructure::config::AppConfig::new_config();
    let _db = infrastructure::db::DB::new_database(&cfg);

    let routing = infrastructure::routing::Routing::new_routing(&cfg);
    routing.run(routing.port);
}