#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
mod controllers;
mod repositories;
mod services;

use repositories::mongo_client::MongoClient;
use rocket_contrib::{json::JsonValue, serve::StaticFiles};
use services::{config::Config, task::TaskService};
use std::sync::Mutex;
use todo_models::Entry;

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

fn rocket() -> rocket::Rocket {
    let entries: Vec<Entry> = Vec::new();
    let config_service = Config::new();
    let mongo_client = MongoClient::new(config_service.clone());

    rocket::ignite()
        .mount(
            "/tasks",
            routes![controllers::task::list, controllers::task::update],
        )
        .mount("/", StaticFiles::from(&config_service.static_files))
        .register(catchers![not_found])
        .manage(Mutex::new(entries))
        .manage(Mutex::new(TaskService::new(mongo_client)))
}

fn main() {
    rocket().launch();
}
