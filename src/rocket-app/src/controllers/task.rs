use crate::TaskService;
use rocket::State;
use rocket_contrib::json::Json;
use std::sync::Mutex;
use todo_models::{Entry, TaskResponse};

type EntryMap = Mutex<Vec<Entry>>;

#[get("/", format = "json")]
pub fn list(
    map: State<EntryMap>,
    task_repo: State<Mutex<TaskService>>,
) -> Option<Json<TaskResponse>> {
    let mut entries = map.lock().unwrap();

    if entries.is_empty() {
        *entries = task_repo.lock().unwrap().get_tasks().unwrap();
    }

    Some(Json(TaskResponse {
        tasks: entries.to_vec(),
    }))
}

#[put("/", format = "json", data = "<request_entries>")]
pub fn update(
    request_entries: Json<TaskResponse>,
    map: State<EntryMap>,
    task_repo: State<Mutex<TaskService>>,
) -> Option<Json<TaskResponse>> {
    let mut entries = map.lock().unwrap();
    let response = task_repo
        .lock()
        .unwrap()
        .update_tasks(request_entries.into_inner().tasks);
    *entries = task_repo.lock().unwrap().get_tasks().unwrap();

    Some(Json(TaskResponse {
        tasks: response.unwrap(),
    }))
}

#[put("/<_entry_id>", format = "json", data = "<entry_body>")]
pub fn update_entry(
    _entry_id: String,
    entry_body: Json<Entry>,
    map: State<EntryMap>,
    task_repo: State<Mutex<TaskService>>,
) -> Option<Json<TaskResponse>> {
    let response = task_repo.lock().unwrap().update_task(Entry {
        _id: entry_body._id.to_string(),
        description: entry_body.description.to_string(),
        completed: entry_body.completed,
        editing: entry_body.editing,
    });

    let updated_entry = response.unwrap();
    let mut entries = map.lock().unwrap();
    let entries_clone = entries.to_vec();
    for (idx, entry) in entries_clone.into_iter().enumerate() {
        if entry._id == updated_entry._id {
            entries.remove(idx);
            entries.insert(idx, updated_entry);
            break;
        }
    }

    Some(Json(TaskResponse {
        tasks: entries.to_vec(),
    }))
}
