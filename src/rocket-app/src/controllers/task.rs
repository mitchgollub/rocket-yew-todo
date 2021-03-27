use crate::TaskService;
use rocket::State;
use rocket_contrib::json::Json;
use std::sync::Mutex;
use todo_models::{Entry, TaskResponse};

type EntryMap = Mutex<Vec<Entry>>;

#[get("/", format = "json")]
pub fn list(task_repo: State<Mutex<TaskService>>) -> Option<Json<TaskResponse>> {
    let response = task_repo.lock().unwrap().get_tasks();
    Some(Json(TaskResponse {
        tasks: response.unwrap(),
    }))
}

#[put("/", format = "json", data = "<request_entries>")]
pub fn update(
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
