use crate::router::payloads::{CommonResponse, GetBuildDatePayload};
use axum::extract::ws::{Message, WebSocket};
use axum::extract::{State, WebSocketUpgrade};
use axum::response::IntoResponse;
use axum::Json;
use colored::*;
use kamura_core::consts::WS_INTERVAL_MILLI_SEC;
use kamura_integrator::Integrator;
use sayaka::debug_fn;
use std::time::Duration;
use tokio::time::sleep;

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
        sleep(Duration::from_millis(WS_INTERVAL_MILLI_SEC)).await;
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
        sleep(Duration::from_millis(WS_INTERVAL_MILLI_SEC)).await;
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
        sleep(Duration::from_millis(WS_INTERVAL_MILLI_SEC)).await;
    }
}