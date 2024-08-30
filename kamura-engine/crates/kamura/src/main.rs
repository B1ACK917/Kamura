mod router;
mod utils;

use crate::router::root;
use crate::utils::cli;
use axum::{
    routing::get,
    Router,
};
use colored::*;
use sayaka::{debug_fn, debug_var};
use std::error::Error;
use std::path::PathBuf;
use tokio::signal;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    debug_fn!();

    let args = cli().get_matches();
    let perseus_path = args.get_one::<PathBuf>("perseus");
    debug_var!(perseus_path);

    let app = Router::new()
        .merge(root());
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("Kamura running on {}", listener.local_addr()?);
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
