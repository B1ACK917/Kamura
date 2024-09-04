use crate::router::auth;
use crate::router::payloads::{AuthorizedPayload, CommonResponse};
use axum::extract::State;
use axum::Json;
use kamura_controller::Controller;
use sayaka::debug_fn;

pub async fn flush_hashset(mut state: State<Controller>, Json(payload): Json<AuthorizedPayload>) -> Json<CommonResponse> {
    debug_fn!();
    if !auth(payload.auth) {
        return Json(CommonResponse { success: false, message: "Authorize Failed".to_string() });
    }
    match state.flush_hashset(&payload.target) {
        Ok(_) => {
            Json(CommonResponse { success: true, message: format!("Flushed Hashset {}", payload.target) })
        }
        Err(err) => {
            Json(CommonResponse { success: false, message: err.to_string() })
        }
    }
}

pub async fn flush_all(mut state: State<Controller>, Json(payload): Json<AuthorizedPayload>) -> Json<CommonResponse> {
    debug_fn!();
    if !auth(payload.auth) {
        return Json(CommonResponse { success: false, message: "Authorize Failed".to_string() });
    }
    match state.flush_all() {
        Ok(_) => {
            Json(CommonResponse { success: true, message: "Flushed All".to_string() })
        }
        Err(err) => {
            Json(CommonResponse { success: false, message: err.to_string() })
        }
    }
}