use axum::extract::ws::{Message, WebSocket};
use axum::extract::{Path, State, WebSocketUpgrade};
use axum::response::IntoResponse;
use axum::Json;
use colored::*;
use kamura_integrator::Integrator;
use kamura_runner::Runner;
use sayaka::debug_fn;
use std::time::Duration;
use tokio::time::sleep;
use crate::router::consts::WS_INTERVAL_MILLI_SEC;
use crate::router::payloads::{AddTaskPayload, CommonResponse, GetBuildDatePayload, GetTaskPayload, Tasks, WorkloadsResponse};

pub async fn root() -> &'static str {
    debug_fn!();
    "Greetings From Kamura!"
}

pub async fn get_valid_workloads(state: State<Runner>) -> Json<WorkloadsResponse> {
    debug_fn!();
    match state.get_valid_workloads() {
        Ok(workloads) => {
            Json(WorkloadsResponse { success: true, workloads })
        }
        Err(_) => {
            Json(WorkloadsResponse { success: false, workloads: Vec::new() })
        }
    }
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

pub async fn get_task_status(ws: WebSocketUpgrade, Path(uuid): Path<String>, state: State<Runner>) -> impl IntoResponse {
    debug_fn!();
    ws.on_upgrade(|socket| get_task_status_handler(socket, state, uuid))
}

async fn get_task_status_handler(mut socket: WebSocket, state: State<Runner>, uuid: String) {
    debug_fn!();
    loop {
        let sent;
        match state.get_task_status(&uuid) {
            Ok(status) => {
                sent = socket.send(Message::Text(status)).await;
            }
            Err(err) => {
                sent = socket
                    .send(Message::Text(format!("Error: {}", err)))
                    .await;
            }
        }
        if sent.is_err() { break; }
        sleep(Duration::from_millis(WS_INTERVAL_MILLI_SEC)).await;
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

pub async fn get_perseus_version(state: State<Integrator>) -> Json<CommonResponse> {
    debug_fn!();
    match state.get_perseus_latest_commit_hash() {
        Ok(head) => {
            Json(CommonResponse { success: true, message: head })
        }
        Err(err) => {
            Json(CommonResponse { success: false, message: err.to_string() })
        }
    }
}

pub async fn get_perseus_date(state: State<Integrator>) -> Json<CommonResponse> {
    debug_fn!();
    match state.get_perseus_latest_commit_date() {
        Ok(head) => {
            Json(CommonResponse { success: true, message: head })
        }
        Err(err) => {
            Json(CommonResponse { success: false, message: err.to_string() })
        }
    }
}

pub async fn get_perseus_path(state: State<Integrator>) -> Json<CommonResponse> {
    debug_fn!();
    Json(CommonResponse { success: true, message: state.get_perseus_path() })
}

pub async fn get_build_date(state: State<Integrator>, Json(payload): Json<GetBuildDatePayload>) -> Json<CommonResponse> {
    debug_fn!(payload);
    match state.get_build_date(payload.module) {
        Ok(date) => {
            Json(CommonResponse { success: true, message: date })
        }
        Err(err) => {
            Json(CommonResponse { success: false, message: err.to_string() })
        }
    }
}

pub async fn get_perseus_status(state: State<Integrator>) -> Json<CommonResponse> {
    debug_fn!();
    match state.check_valid() {
        true => {
            Json(CommonResponse { success: true, message: "Valid".to_string() })
        }
        false => {
            Json(CommonResponse { success: false, message: "Invalid".to_string() })
        }
    }
}

pub async fn rebuild_perseus(state: State<Integrator>) -> Json<CommonResponse> {
    debug_fn!();
    match state.rebuild_perseus() {
        Ok(_) => {
            Json(CommonResponse { success: true, message: "".to_string() })
        }
        Err(err) => {
            Json(CommonResponse { success: false, message: err.to_string() })
        }
    }
}

pub async fn get_perseus_rebuild_status(ws: WebSocketUpgrade, state: State<Integrator>) -> impl IntoResponse {
    debug_fn!();
    ws.on_upgrade(|socket| get_perseus_rebuild_status_handler(socket, state))
}

async fn get_perseus_rebuild_status_handler(mut socket: WebSocket, state: State<Integrator>) {
    debug_fn!();
    loop {
        let data;
        match state.get_perseus_rebuild_status() {
            Ok(status) => {
                if status.starts_with("Failed") {
                    data = serde_json::to_string(&CommonResponse { success: false, message: status }).unwrap();
                } else {
                    data = serde_json::to_string(&CommonResponse { success: true, message: status }).unwrap();
                }
            }
            Err(err) => {
                data = serde_json::to_string(&CommonResponse { success: false, message: err.to_string() }).unwrap();
            }
        }
        let sent = socket.send(Message::Text(data)).await;
        if sent.is_err() { break; }
        sleep(Duration::from_secs(WS_INTERVAL_MILLI_SEC)).await;
    }
}

pub async fn update_perseus(state: State<Integrator>) -> Json<CommonResponse> {
    debug_fn!();
    match state.update_perseus() {
        Ok(_) => {
            Json(CommonResponse { success: true, message: "".to_string() })
        }
        Err(err) => {
            Json(CommonResponse { success: false, message: err.to_string() })
        }
    }
}

pub async fn get_perseus_update_status(ws: WebSocketUpgrade, state: State<Integrator>) -> impl IntoResponse {
    debug_fn!();
    ws.on_upgrade(|socket| get_perseus_update_status_handler(socket, state))
}

async fn get_perseus_update_status_handler(mut socket: WebSocket, state: State<Integrator>) {
    debug_fn!();
    loop {
        let data;
        match state.get_perseus_update_status() {
            Ok(status) => {
                if status.starts_with("Failed") {
                    data = serde_json::to_string(&CommonResponse { success: false, message: status }).unwrap();
                } else {
                    data = serde_json::to_string(&CommonResponse { success: true, message: status }).unwrap();
                }
            }
            Err(err) => {
                data = serde_json::to_string(&CommonResponse { success: false, message: err.to_string() }).unwrap();
            }
        }
        let sent = socket.send(Message::Text(data)).await;
        if sent.is_err() { break; }
        sleep(Duration::from_secs(WS_INTERVAL_MILLI_SEC)).await;
    }
}

pub async fn rebuild_spike(state: State<Integrator>) -> Json<CommonResponse> {
    debug_fn!();
    match state.rebuild_spike() {
        Ok(_) => {
            Json(CommonResponse { success: true, message: "".to_string() })
        }
        Err(err) => {
            Json(CommonResponse { success: false, message: err.to_string() })
        }
    }
}

pub async fn get_spike_rebuild_status(ws: WebSocketUpgrade, state: State<Integrator>) -> impl IntoResponse {
    debug_fn!();
    ws.on_upgrade(|socket| get_spike_rebuild_status_handler(socket, state))
}

async fn get_spike_rebuild_status_handler(mut socket: WebSocket, state: State<Integrator>) {
    debug_fn!();
    loop {
        let data;
        match state.get_spike_rebuild_status() {
            Ok(status) => {
                if status.starts_with("Failed") {
                    data = serde_json::to_string(&CommonResponse { success: false, message: status }).unwrap();
                } else {
                    data = serde_json::to_string(&CommonResponse { success: true, message: status }).unwrap();
                }
            }
            Err(err) => {
                data = serde_json::to_string(&CommonResponse { success: false, message: err.to_string() }).unwrap();
            }
        }
        let sent = socket.send(Message::Text(data)).await;
        if sent.is_err() { break; }
        sleep(Duration::from_secs(WS_INTERVAL_MILLI_SEC)).await;
    }
}
