mod router;
mod utils;

use crate::router::{add_task, flush_all, get_all_tasks, root};
use crate::utils::cli;
use axum::routing::{get, post};
use axum::Router;
use colored::*;
use kamura_runner::Runner;
use sayaka::debug_fn;
use std::error::Error;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    debug_fn!();

    let args = cli().get_matches();
    let perseus_path = args.get_one::<PathBuf>("perseus").unwrap();
    let redis = args.get_one::<String>("redis").unwrap();
    let bind_address = args.get_one::<String>("bind").unwrap();

    let kamura_runner = Runner::new(perseus_path, redis)?;

    let app = Router::new()
        .route("/", get(root))
        .route("/addTask", post(add_task))
        .route("/getAllTasks", get(get_all_tasks))
        .route("/flushAll", get(flush_all))
        .with_state(kamura_runner);
    let listener = tokio::net::TcpListener::bind(bind_address).await?;
    println!("Kamura running on {}", listener.local_addr()?);
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
