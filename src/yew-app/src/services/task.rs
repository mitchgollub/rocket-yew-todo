use serde_derive::{Deserialize, Serialize};
use yewtil::fetch::{Fetch, FetchAction, FetchRequest, Json, MethodBody};

pub type TaskRequest = Fetch<GetTaskRequest, RequestBody>;

pub type UpdateTaskRequest = Fetch<PutTaskRequest, RequestBody>;

pub type TaskFetchAction = FetchAction<RequestBody>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
    pub description: String,
    pub completed: bool,
    pub editing: bool,
}

#[derive(Default, Debug, Clone)]
pub struct GetTaskRequest;

#[derive(Default, Debug)]
pub struct PutTaskRequest {
    pub data: RequestBody,
}

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[derive(Debug, Serialize, Deserialize)]
pub struct RequestBody {
    pub tasks: Vec<Entry>,
}

impl Default for RequestBody {
    fn default() -> RequestBody {
        RequestBody { tasks: Vec::new() }
    }
}

impl FetchRequest for GetTaskRequest {
    type RequestBody = ();
    type ResponseBody = RequestBody;
    type Format = Json;

    fn url(&self) -> String {
        // Given that this is an external resource, this may fail sometime in the future.
        // Please report any regressions related to this.
        "http://localhost:8000/tasks".to_string()
    }

    fn method(&self) -> MethodBody<Self::RequestBody> {
        MethodBody::Get
    }

    fn headers(&self) -> Vec<(String, String)> {
        vec![]
    }

    fn use_cors(&self) -> bool {
        true
    }
}

impl FetchRequest for PutTaskRequest {
    type RequestBody = RequestBody;
    type ResponseBody = RequestBody;
    type Format = Json;

    fn url(&self) -> String {
        // Given that this is an external resource, this may fail sometime in the future.
        // Please report any regressions related to this.
        "http://localhost:8000/tasks".to_string()
    }

    fn method(&self) -> MethodBody<Self::RequestBody> {
        MethodBody::Put(&self.data)
    }

    fn headers(&self) -> Vec<(String, String)> {
        vec![("Content-Type".to_string(), "application/json".to_string())]
    }

    fn use_cors(&self) -> bool {
        true
    }
}