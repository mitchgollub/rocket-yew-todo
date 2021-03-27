use crate::Model;
use crate::Msg;
use todo_models::{TaskRequest, TaskResponse};
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::ComponentLink;

const TASK_API_URL: &str = "/tasks";

pub struct TaskClient {}

impl TaskClient {
    pub fn get_tasks(link: &ComponentLink<Model>) -> Option<FetchTask> {
        let request = Request::get(TASK_API_URL)
            .body(Nothing)
            .expect("Could not build request.");
        let callback = link.callback(
            |response: Response<Json<Result<TaskResponse, anyhow::Error>>>| {
                let Json(data) = response.into_body();
                Msg::TasksReceived(data)
            },
        );
        Some(FetchService::fetch(request, callback).expect("failed to start request"))
    }

    pub fn update_tasks(link: &ComponentLink<Model>, tasks: &TaskRequest) -> Option<FetchTask> {
        let request = Request::put(TASK_API_URL)
            .header("Content-Type", "application/json")
            .body(Json(tasks))
            .expect("Could not build request.");
        let callback = link.callback(
            |response: Response<Json<Result<TaskResponse, anyhow::Error>>>| {
                let Json(data) = response.into_body();
                Msg::TasksReceived(data)
            },
        );
        Some(FetchService::fetch(request, callback).expect("failed to start request"))
    }

    // pub fn delete_all(link: &ComponentLink<Model>) -> Option<FetchTask> {
    //     let request = Request::delete(TASK_API_URL)
    //         .header("Content-Type", "application/json")
    //         .body(Nothing)
    //         .expect("Could not build request.");
    //     let callback = link.callback(|response: Response<Result<String, anyhow::Error>>| {
    //         if response.status() == 204 {
    //             Msg::JobsReceived(Ok(TaskResponse { jobs: Vec::new() }))
    //         } else {
    //             ConsoleService::error(&format!("received from delete: {:?}", response.status()));
    //             Msg::Noop
    //         }
    //     });
    //     Some(FetchService::fetch(request, callback).expect("failed to start request"))
    // }
}
