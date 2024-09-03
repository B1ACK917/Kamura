mod router;
mod utils;

use crate::router::{add_task, flush_all, get_all_tasks, get_build_date, get_perseus_date, get_perseus_path, get_perseus_rebuild_status, get_perseus_status, get_perseus_update_status, get_perseus_version, get_raw_arch, get_spike_rebuild_status, get_task_log, get_task_status, get_valid_workloads, list_arches, rebuild_perseus, rebuild_spike, remove_all_tasks, root, update_perseus};
use crate::utils::cli;
use axum::routing::{get, post};
use axum::Router;
use kamura_integrator::Integrator;
use kamura_operator::Operator;
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
    let kamura_operator = Operator::new(perseus_path, redis)?;

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/", get(root))
        .route("/getValidWorkloads", get(get_valid_workloads))
        .route("/addTask", post(add_task))
        .route("/getTaskLog", post(get_task_log))
        .route("/getAllTasks", get(get_all_tasks))
        .route("/removeAllTasks", post(remove_all_tasks))
        .route("/ws/getTaskStatus/:uuid", get(get_task_status))
        .with_state(kamura_runner)
        .route("/getPerseus", get(get_perseus_path))
        .route("/getPerseusVersion", get(get_perseus_version))
        .route("/getPerseusDate", get(get_perseus_date))
        .route("/getPerseusStatus", get(get_perseus_status))
        .route("/getBuildDate", post(get_build_date))
        .route("/rebuildPerseus", get(rebuild_perseus))
        .route("/ws/getPerseusRebuildStatus", get(get_perseus_rebuild_status))
        .route("/updatePerseus", get(update_perseus))
        .route("/ws/getPerseusUpdateStatus", get(get_perseus_update_status))
        .route("/rebuildSpike", get(rebuild_spike))
        .route("/ws/getSpikeRebuildStatus", get(get_spike_rebuild_status))
        .with_state(kamura_integrator)
        .route("/listArches", get(list_arches))
        .route("/getRawArch", post(get_raw_arch))
        .route("/flushAll", post(flush_all))
        .with_state(kamura_operator)
        .layer(cors);
    let listener = tokio::net::TcpListener::bind(bind_address).await?;
    println!("Kamura running on {}", listener.local_addr()?);
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
