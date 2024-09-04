use crate::router::payloads::{AddTaskPayload, CommonResponse, GetTaskPayload, Tasks, Workloads};
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
