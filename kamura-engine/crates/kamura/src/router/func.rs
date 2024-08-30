use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use sayaka::debug_fn;
use colored::*;

pub async fn root() -> &'static str {
    debug_fn!();
    "Kamura Engine Ready"
}