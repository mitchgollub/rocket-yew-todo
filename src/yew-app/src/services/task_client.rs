use crate::Model;
use crate::Msg;
use todo_models::Entry;
use todo_models::{TaskRequest, TaskResponse};
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::ComponentLink;

const TASK_API_URL: &str = "/tasks";

pub struct TaskClient {}

impl TaskClient {
    pub fn get_tasks(link: &ComponentLink<Model>, request_id: u64) -> FetchTask {
        let request = Request::get(TASK_API_URL)
            .body(Nothing)
            .expect("Could not build request.");
        let callback = link.callback(
            move |response: Response<Json<Result<TaskResponse, anyhow::Error>>>| {
                let Json(data) = response.into_body();
                Msg::TasksReceived(request_id, data)
            },
        );
        FetchService::fetch(request, callback).expect("failed to start request")
    }

    pub fn update_tasks(
        link: &ComponentLink<Model>,
        request_id: u64,
        tasks: &TaskRequest,
    ) -> FetchTask {
        let request = Request::put(TASK_API_URL)
            .header("Content-Type", "application/json")
            .body(Json(tasks))
            .expect("Could not build request.");
        let callback = link.callback(
            move |response: Response<Json<Result<TaskResponse, anyhow::Error>>>| {
                let Json(data) = response.into_body();
                Msg::TasksReceived(request_id, data)
            },
        );
        FetchService::fetch(request, callback).expect("failed to start request")
    }

    pub fn update_task(link: &ComponentLink<Model>, request_id: u64, entry: &Entry) -> FetchTask {
        let entry_id = match entry._id.is_empty() {
            true => "0",
            false => &entry._id,
        };
        let request = Request::put(format!("{}/{}", TASK_API_URL, entry_id))
            .header("Content-Type", "application/json")
            .body(Json(entry))
            .expect("Could not build request.");
        let callback = link.callback(
            move |response: Response<Json<Result<Entry, anyhow::Error>>>| {
                let Json(data) = response.into_body();
                Msg::UpdateEntryReceived(request_id, data)
            },
        );
        FetchService::fetch(request, callback).expect("failed to start request")
    }

    pub fn add_task(link: &ComponentLink<Model>, request_id: u64, entry: &Entry) -> FetchTask {
        let request = Request::post(format!("{}", TASK_API_URL))
            .header("Content-Type", "application/json")
            .body(Json(entry))
            .expect("Could not build request.");
        let callback = link.callback(
            move |response: Response<Json<Result<Entry, anyhow::Error>>>| {
                let Json(data) = response.into_body();
                Msg::AddEntryReceived(request_id, data)
            },
        );
        FetchService::fetch(request, callback).expect("failed to start request")
    }

    pub fn delete_task(link: &ComponentLink<Model>, request_id: u64, entry: &Entry) -> FetchTask {
        let request = Request::delete(format!("{}/{}", TASK_API_URL, entry._id))
            .header("Content-Type", "application/json")
            .body(Nothing)
            .expect("Could not build request.");
        let callback = link.callback(
            move |response: Response<Json<Result<String, anyhow::Error>>>| {
                let Json(data) = response.into_body();
                Msg::DeleteEntryReceived(request_id, data)
            },
        );
        FetchService::fetch(request, callback).expect("failed to start request")
    }
}
