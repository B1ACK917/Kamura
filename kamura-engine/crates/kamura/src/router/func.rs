use axum::Router;
use axum::routing::{get, MethodRouter};

fn route(path: &str, method_router: MethodRouter<()>) -> Router {
    Router::new().route(path, method_router)
}

pub fn root() -> Router {
    async fn handler() -> &'static str {
        "Greetings From Kamura!"
    }

    route("/", get(handler))
}