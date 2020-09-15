#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

#[cfg(test)]
mod tests;

use std::sync::Mutex;

use rocket::State;
use rocket_contrib::json::{Json, JsonValue};
use rocket_contrib::serve::StaticFiles;

// We're going to store all of the entrys here. No need for a DB.
type EntryMap = Mutex<Vec<Entry>>;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Entry {
    description: String,
    completed: bool,
    editing: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskResponse {
    tasks: Vec<Entry>,
}

// // TODO: This example can be improved by using `route` with multiple HTTP verbs.
// #[post("/<id>", format = "json", data = "<entry>")]
// fn new(id: ID, entry: Json<Entry>, map: State<EntryMap>) -> JsonValue {
//     let mut entries = map.lock().expect("map lock.");
//     if entries.contains_key(&id) {
//         json!({
//             "status": "error",
//             "reason": "ID exists. Try put."
//         })
//     } else {
//         entries.insert(id, entry.0.contents);
//         json!({ "status": "ok" })
//     }
// }

// #[put("/<id>", format = "json", data = "<entry>")]
// fn update(id: ID, entry: Json<Entry>, map: State<EntryMap>) -> Option<JsonValue> {
//     let mut entries = map.lock().unwrap();
//     if entries.contains_key(&id) {
//         entries.insert(id, entry.0.contents);
//         Some(json!({ "status": "ok" }))
//     } else {
//         None
//     }
// }

// #[get("/<id>", format = "json")]
// fn get(id: ID, map: State<EntryMap>) -> Option<Json<Entry>> {
//     let entries = map.lock().unwrap();
//     entries.get(&id).map(|contents| {
//         Json(Entry {
//             id: Some(id),
//             contents: contents.clone(),
//         })
//     })
// }

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
            description: val.description.to_string(),
            completed: val.completed,
            editing: val.editing,
        });
    }
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
    let mut entries = Vec::new();
    entries.push(Entry {
        description: "stuff".to_string(),
        completed: false,
        editing: false,
    });
    rocket::ignite()
        //.mount("/tasks", routes![new, update, get])
        .mount("/tasks", routes![list, update])
        .mount("/", StaticFiles::from("../../dist"))
        .register(catchers![not_found])
        .manage(Mutex::new(entries))
}

fn main() {
    rocket().launch();
}
