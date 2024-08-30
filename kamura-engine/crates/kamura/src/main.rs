mod router;

use crate::router::root;
use axum::{
    routing::{get, post},
    Router,
};
use colored::*;
use sayaka::debug_fn;

#[tokio::main]
async fn main() {
    debug_fn!();

    let app = Router::new()
        .route("/", get(root));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
