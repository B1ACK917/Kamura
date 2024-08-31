use axum::extract::State;
use axum::Json;
use colored::*;
use kamura_runner::Runner;
use sayaka::debug_fn;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct CommonResponse {
    success: bool,
    message: String,
}
#[derive(Serialize)]
pub struct Tasks {
    success: bool,
    tasks: Vec<String>,
    message: String,
}

#[derive(Deserialize, Debug)]
pub struct AddTaskPayload {
    arch: String,
    workload: String,
    workload_type: String,
}

#[derive(Deserialize, Debug)]
pub struct GetTaskPayload {
    uuid: String,
}

pub async fn root() -> &'static str {
    debug_fn!();
    "Greetings From Kamura!"
}

pub async fn add_task(mut state: State<Runner>, Json(payload): Json<AddTaskPayload>) -> Json<CommonResponse> {
    debug_fn!(payload);
    match state.add_task(&payload.arch, &payload.workload, &payload.workload_type).await {
        Ok(uuid) => {
            Json(CommonResponse { success: true, message: uuid.to_string() })
        }
        Err(err) => {
            Json(CommonResponse { success: false, message: err.to_string() })
        }
    }
}

pub async fn get_task_log(state: State<Runner>, Json(payload): Json<GetTaskPayload>) -> Json<CommonResponse> {
    debug_fn!(payload);
    match state.get_task_log(&payload.uuid) {
        Ok(content) => {
            Json(CommonResponse { success: true, message: content })
        }
        Err(err) => {
            Json(CommonResponse { success: false, message: err.to_string() })
        }
    }
}

pub async fn get_task_status(state: State<Runner>, Json(payload): Json<GetTaskPayload>) -> Json<CommonResponse> {
    debug_fn!(payload);
    match state.get_task_status(&payload.uuid) {
        Ok(content) => {
            Json(CommonResponse { success: true, message: content })
        }
        Err(err) => {
            Json(CommonResponse { success: false, message: err.to_string() })
        }
    }
}

pub async fn get_all_tasks(mut state: State<Runner>) -> Json<Tasks> {
    debug_fn!();
    match state.get_all_tasks() {
        Ok(tasks) => {
            Json(Tasks { success: true, tasks, message: "".to_string() })
        }
        Err(err) => {
            Json(Tasks { success: false, tasks: Vec::new(), message: err.to_string() })
        }
    }
}

pub async fn flush_all(mut state: State<Runner>) -> Json<CommonResponse> {
    debug_fn!();
    match state.flush_all() {
        Ok(_) => {
            Json(CommonResponse { success: true, message: "Flushed All Redis".to_string() })
        }
        Err(err) => {
            Json(CommonResponse { success: false, message: err.to_string() })
        }
    }
}
