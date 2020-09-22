#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
mod repositories;

use repositories::task::{update_tasks, Entry};
use rocket::State;
use rocket_contrib::databases;
use rocket_contrib::json::{Json, JsonValue};
use rocket_contrib::serve::StaticFiles;
use std::sync::Mutex;

type EntryMap = Mutex<Vec<Entry>>;

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskResponse {
    tasks: Vec<Entry>,
}

#[get("/", format = "json")]
fn list(map: State<EntryMap>) -> Option<Json<TaskResponse>> {
    let entries = map.lock().unwrap();
    Some(Json(TaskResponse {
        tasks: entries.to_vec(),
    }))
}

#[put("/", format = "json", data = "<request_entries>")]
fn update(request_entries: Json<TaskResponse>, map: State<EntryMap>) -> Option<Json<TaskResponse>> {
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

    update_tasks(entries.to_vec());

    Some(Json(TaskResponse {
        tasks: entries.to_vec(),
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

    rocket::ignite()
        .mount("/tasks", routes![list, update])
        .mount("/", StaticFiles::from("../../dist"))
        .register(catchers![not_found])
        .manage(Mutex::new(entries))
}

fn main() {
    rocket().launch();
}
