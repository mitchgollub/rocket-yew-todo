use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskResponse {
    pub tasks: Vec<Entry>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Entry {
    #[serde(default)]
    pub _id: String,
    pub completed: bool,
    pub description: String,
    pub editing: bool,
}
