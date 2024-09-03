use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct CommonResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Serialize)]
pub struct WorkloadsResponse {
    pub success: bool,
    pub workloads: Vec<[String; 2]>,
}

#[derive(Serialize)]
pub struct Tasks {
    pub success: bool,
    pub tasks: Vec<String>,
    pub message: String,
}

#[derive(Deserialize, Debug)]
pub struct AddTaskPayload {
    pub arch: String,
    pub workload: String,
    pub workload_type: String,
}

#[derive(Deserialize, Debug)]
pub struct GetTaskPayload {
    pub uuid: String,
}

#[derive(Deserialize, Debug)]
pub struct GetBuildDatePayload {
    pub module: String,
}

#[derive(Deserialize, Debug)]
pub struct AuthorizedPayload {
    pub auth: String,
}