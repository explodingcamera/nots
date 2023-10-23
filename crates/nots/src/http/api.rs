use axum::routing::get;
use axum::Router;

use crate::state::AppState;

pub fn new(app_state: AppState) -> Router {
    Router::new().route("/", get(hi))
}

async fn hi() -> &'static str {
    "Hello, World!"
}
