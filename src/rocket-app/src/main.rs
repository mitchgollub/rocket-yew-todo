#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
mod repositories;
mod services;

use repositories::mongo_client::MongoClient;
use rocket::State;
use rocket_contrib::{
    json::{Json, JsonValue},
    serve::StaticFiles,
};
use services::{config::Config, task::TaskService};
use std::sync::Mutex;
use todo_models::{Entry, TaskResponse};

type EntryMap = Mutex<Vec<Entry>>;

#[get("/", format = "json")]
fn list(task_repo: State<Mutex<TaskService>>) -> Option<Json<TaskResponse>> {
    let response = task_repo.lock().unwrap().get_tasks();
    Some(Json(TaskResponse {
        tasks: response.unwrap(),
    }))
}

#[put("/", format = "json", data = "<request_entries>")]
fn update(
    request_entries: Json<TaskResponse>,
    map: State<EntryMap>,
    task_repo: State<Mutex<TaskService>>,
) -> Option<Json<TaskResponse>> {
    let mut entries = map.lock().unwrap();
    entries.clear();
    for val in request_entries.tasks.iter() {
        entries.push(Entry {
            _id: val._id.to_string(),
            description: val.description.to_string(),
            completed: val.completed,
            editing: val.editing,
        });
    }

    let response = task_repo.lock().unwrap().update_tasks(entries.to_vec());

    Some(Json(TaskResponse {
        tasks: response.unwrap(),
    }))
}

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
        .mount("/tasks", routes![list, update])
        .mount("/", StaticFiles::from(&config_service.static_files))
        .register(catchers![not_found])
        .manage(Mutex::new(entries))
        .manage(Mutex::new(TaskService::new(mongo_client)))
}

fn main() {
    rocket().launch();
}
