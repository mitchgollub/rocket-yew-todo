use crate::TaskService;
use rocket::State;
use rocket_contrib::json::Json;
use std::sync::Mutex;
use todo_models::{Entry, TaskResponse};

#[get("/", format = "json")]
pub fn list(task_repo: State<Mutex<TaskService>>) -> Option<Json<TaskResponse>> {
    let entries = task_repo.lock().unwrap().get_tasks().unwrap();

    Some(Json(TaskResponse { tasks: entries }))
}

#[put("/", format = "json", data = "<request_entries>")]
pub fn update(
    request_entries: Json<TaskResponse>,
    task_repo: State<Mutex<TaskService>>,
) -> Option<Json<TaskResponse>> {
    let response = task_repo
        .lock()
        .unwrap()
        .update_tasks(request_entries.into_inner().tasks)
        .unwrap();

    Some(Json(TaskResponse { tasks: response }))
}

#[put("/<_entry_id>", format = "json", data = "<entry_body>")]
pub fn update_entry(
    _entry_id: String,
    entry_body: Json<Entry>,
    task_repo: State<Mutex<TaskService>>,
) -> Option<Json<Entry>> {
    let response = task_repo
        .lock()
        .unwrap()
        .update_task(entry_body.into_inner())
        .unwrap();

    Some(Json(response))
}

#[post("/", format = "json", data = "<entry_body>")]
pub fn add_entry(
    entry_body: Json<Entry>,
    task_repo: State<Mutex<TaskService>>,
) -> Option<Json<Entry>> {
    let response = task_repo
        .lock()
        .unwrap()
        .add_task(entry_body.into_inner())
        .unwrap();

    Some(Json(response))
}

#[delete("/<entry_id>", format = "json")]
pub fn delete_entry(
    entry_id: String,
    task_repo: State<Mutex<TaskService>>,
) -> Option<Json<String>> {
    let response = task_repo.lock().unwrap().delete_task(entry_id).unwrap();

    Some(Json(response))
}
