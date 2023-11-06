use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{debug_handler, Router};

use crate::state::AppState;

pub fn new(app_state: AppState) -> Router {
    Router::new()
        .route("/", get(hi))
        .route("/app", post(create_app))
        .route("/app/:id", post(update_app))
        .route("/app/:id", get(get_app))
        .route("/apps", get(get_apps))
        .with_state(app_state)
}

async fn create_app(State(app): State<AppState>, body: String) -> impl IntoResponse {
    unimplemented!()
}

async fn update_app(State(app): State<AppState>, body: String) -> impl IntoResponse {
    unimplemented!()
}

async fn get_app(State(app): State<AppState>, body: String) -> impl IntoResponse {
    unimplemented!()
}

async fn get_apps(State(app): State<AppState>, body: String) -> impl IntoResponse {
    unimplemented!()
}

async fn hi() -> &'static str {
    "Hello, World!"
}
