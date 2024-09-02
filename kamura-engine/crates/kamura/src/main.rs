mod router;
mod utils;

use crate::router::{add_task, flush_all, get_all_tasks, get_perseus_build_date, get_perseus_path, get_perseus_rebuild_status, get_perseus_status, get_perseus_version, get_task_log, get_task_status, rebuild_perseus, root};
use crate::utils::cli;
use axum::routing::{get, post};
use axum::Router;
use colored::*;
use kamura_integrator::Integrator;
use kamura_runner::Runner;
use sayaka::debug_fn;
use std::error::Error;
use std::path::PathBuf;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    debug_fn!();

    let args = cli().get_matches();
    let perseus_path = args.get_one::<PathBuf>("perseus").unwrap();
    let redis = args.get_one::<String>("redis").unwrap();
    let bind_address = args.get_one::<String>("bind").unwrap();

    let kamura_runner = Runner::new(perseus_path, redis)?;
    let kamura_integrator = Integrator::new(perseus_path, redis)?;

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/", get(root))
        .route("/addTask", post(add_task))
        .route("/getTaskStatus", post(get_task_status))
        .route("/getTaskLog", post(get_task_log))
        .route("/getAllTasks", get(get_all_tasks))
        .route("/flushAll", get(flush_all))
        .with_state(kamura_runner)
        .route("/getPerseus", get(get_perseus_path))
        .route("/getPerseusVersion", get(get_perseus_version))
        .route("/getPerseusStatus", get(get_perseus_status))
        .route("/getPerseusBuildDate", get(get_perseus_build_date))
        .route("/rebuildPerseus", get(rebuild_perseus))
        .route("/getPerseusRebuildStatus", get(get_perseus_rebuild_status))
        .with_state(kamura_integrator)
        .layer(cors);
    let listener = tokio::net::TcpListener::bind(bind_address).await?;
    println!("Kamura running on {}", listener.local_addr()?);
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
