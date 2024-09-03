use crate::router::auth;
use crate::router::payloads::{Arches, AuthorizedPayload, CommonResponse};
use axum::extract::State;
use axum::Json;
use colored::*;
use kamura_operator::Operator;
use sayaka::debug_fn;

pub async fn flush_all(mut state: State<Operator>, Json(payload): Json<AuthorizedPayload>) -> Json<CommonResponse> {
    debug_fn!();
    if !auth(payload.auth) {
        return Json(CommonResponse { success: false, message: "Authorize Failed".to_string() });
    }
    match state.flush_all() {
        Ok(_) => {
            Json(CommonResponse { success: true, message: "Flushed All Redis".to_string() })
        }
        Err(err) => {
            Json(CommonResponse { success: false, message: err.to_string() })
        }
    }
}

pub async fn list_arches(state: State<Operator>) -> Json<Arches> {
    debug_fn!();
    match state.list_arches() {
        Ok(arches) => {
            Json(Arches { success: true, arches, message: "".to_string() })
        }
        Err(err) => {
            Json(Arches { success: false, arches: Vec::new(), message: err.to_string() })
        }
    }
}