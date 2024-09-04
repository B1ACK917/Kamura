use kamura_operator::{Topology, Units};
use serde::{Deserialize, Serialize};
use serde_json::Value;

// OUT PAYLOADS
#[derive(Serialize, Debug)]
pub struct CommonResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Serialize)]
pub struct Workloads {
    pub success: bool,
    pub workloads: Vec<[String; 2]>,
}

#[derive(Serialize)]
pub struct Tasks {
    pub success: bool,
    pub tasks: Vec<String>,
    pub message: String,
}

#[derive(Serialize)]
pub struct ArchList {
    pub success: bool,
    pub arches: Vec<String>,
    pub message: String,
}

#[derive(Serialize)]
pub struct RawArch {
    pub success: bool,
    pub units: Units,
    pub topology: Topology,
    pub message: String,
}

#[derive(Serialize)]
pub struct Arch {
    pub success: bool,
    pub units: Units,
    pub topology: Topology,
    pub elements: Vec<Value>,
    pub message: String,
}

// IN PAYLOADS

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

#[derive(Deserialize, Debug)]
pub struct GetArchPayload {
    pub target: String,
    pub reset: bool,
}

#[derive(Deserialize, Debug)]
pub struct SaveArchPayload {
    pub target: String,
    pub units: Units,
    pub topology: Topology,
    pub elements: Vec<Value>,
}