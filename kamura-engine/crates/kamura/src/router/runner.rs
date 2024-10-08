use crate::router::payloads::{AddTaskPayload, CommonResponse, TaskLogInfo, Tasks, UniversalTargetPayload, Workloads};
use axum::extract::ws::{Message, WebSocket};
use axum::extract::{Path, State, WebSocketUpgrade};
use axum::response::IntoResponse;
use axum::Json;
use kamura_core::consts::WS_INTERVAL_MILLI_SEC;
use kamura_runner::Runner;
use sayaka::debug_fn;
use std::time::Duration;
use tokio::time::sleep;

pub async fn root() -> &'static str {
    debug_fn!();
    "Greetings From Kamura!"
}

pub async fn get_valid_workloads(state: State<Runner>) -> Json<Workloads> {
    debug_fn!();
    match state.get_valid_workloads() {
        Ok(workloads) => {
            Json(Workloads { success: true, workloads })
        }
        Err(_) => {
            Json(Workloads { success: false, workloads: Vec::new() })
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

pub async fn get_task_info(state: State<Runner>, Json(payload): Json<UniversalTargetPayload>) -> Json<TaskLogInfo> {
    debug_fn!(payload);
    match state.get_task_info(&payload.target) {
        Ok(info) => {
            Json(TaskLogInfo {
                success: true,
                arch: info[0].clone(),
                workload: info[1].clone(),
                submit_time: info[2].clone(),
                finished_time: info[3].clone(),
                elapsed: info[4].clone(),
            })
        }
        Err(_) => {
            Json(TaskLogInfo {
                success: false,
                arch: "".to_string(),
                workload: "".to_string(),
                submit_time: "".to_string(),
                finished_time: "".to_string(),
                elapsed: "".to_string(),
            })
        }
    }
}

pub async fn get_task_log(ws: WebSocketUpgrade, Path(uuid): Path<String>, state: State<Runner>) -> impl IntoResponse {
    debug_fn!();
    ws.on_upgrade(|socket| get_task_log_handler(socket, state, uuid))
}

async fn get_task_log_handler(mut socket: WebSocket, state: State<Runner>, uuid: String) {
    debug_fn!();
    loop {
        let sent;
        match state.get_task_log(&uuid) {
            Ok(content) => {
                sent = socket.send(Message::Text(content)).await;
            }
            Err(err) => {
                sent = socket.send(Message::Text(format!("Error: {}", err))).await;
            }
        }
        if sent.is_err() { break; }
        sleep(Duration::from_millis(WS_INTERVAL_MILLI_SEC)).await;
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

pub async fn remove_task(mut state: State<Runner>, Json(payload): Json<UniversalTargetPayload>) -> Json<CommonResponse> {
    debug_fn!(payload);
    match state.remove_task(&payload.target) {
        Ok(_) => {
            Json(CommonResponse { success: true, message: "".to_string() })
        }
        Err(err) => {
            Json(CommonResponse { success: false, message: err.to_string() })
        }
    }
}