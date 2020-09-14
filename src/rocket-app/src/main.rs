#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

#[cfg(test)]
mod tests;

use std::collections::HashMap;
use std::sync::Mutex;

use rocket::State;
use rocket_contrib::json::{Json, JsonValue};
use rocket_contrib::serve::StaticFiles;

// The type to represent the ID of a entry.
type ID = usize;

// We're going to store all of the entrys here. No need for a DB.
type EntryMap = Mutex<HashMap<ID, Entry>>;

#[derive(Debug, Serialize, Deserialize)]
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
//     let mut hashmap = map.lock().expect("map lock.");
//     if hashmap.contains_key(&id) {
//         json!({
//             "status": "error",
//             "reason": "ID exists. Try put."
//         })
//     } else {
//         hashmap.insert(id, entry.0.contents);
//         json!({ "status": "ok" })
//     }
// }

// #[put("/<id>", format = "json", data = "<entry>")]
// fn update(id: ID, entry: Json<Entry>, map: State<EntryMap>) -> Option<JsonValue> {
//     let mut hashmap = map.lock().unwrap();
//     if hashmap.contains_key(&id) {
//         hashmap.insert(id, entry.0.contents);
//         Some(json!({ "status": "ok" }))
//     } else {
//         None
//     }
// }

// #[get("/<id>", format = "json")]
// fn get(id: ID, map: State<EntryMap>) -> Option<Json<Entry>> {
//     let hashmap = map.lock().unwrap();
//     hashmap.get(&id).map(|contents| {
//         Json(Entry {
//             id: Some(id),
//             contents: contents.clone(),
//         })
//     })
// }

#[get("/", format = "json")]
fn list(map: State<EntryMap>) -> Option<Json<TaskResponse>> {
    let hashmap = map.lock().unwrap();
    hashmap.get(&1).map(|entry| {
        Json(TaskResponse {
            tasks: vec![Entry {
                description: entry.description.to_string(),
                completed: entry.completed,
                editing: entry.editing,
            }],
        })
    })
}

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

fn rocket() -> rocket::Rocket {
    let mut map = HashMap::<ID, Entry>::new();
    map.insert(
        1,
        Entry {
            description: "stuff".to_string(),
            completed: false,
            editing: false,
        },
    );
    rocket::ignite()
        //.mount("/tasks", routes![new, update, get])
        .mount("/tasks", routes![list])
        .mount("/", StaticFiles::from("../../dist"))
        .register(catchers![not_found])
        .manage(Mutex::new(map))
}

fn main() {
    rocket().launch();
}
