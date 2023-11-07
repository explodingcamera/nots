use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use nots_client::api::{CreateAppRequest, ServerStatus};

use crate::state::AppState;

pub fn new(app_state: AppState) -> Router {
    Router::new()
        .route("/", get(hi))
        .route("/status", get(server_status))
        .route("/app", post(create_app))
        .route("/app/:id", post(update_app))
        .route("/app/:id", get(get_app))
        .route("/apps", get(get_apps))
        .with_state(app_state)
}

async fn server_status(State(app): State<AppState>) -> Json<ServerStatus> {
    let uptime = time::OffsetDateTime::now_utc() - app.stated_at;

    Json(ServerStatus {
        version: env!("CARGO_PKG_VERSION").to_string(),
        uptime_secs: uptime.whole_seconds() as u64,
    })
}

async fn create_app(
    State(app): State<AppState>,
    body: Json<CreateAppRequest>,
) -> impl IntoResponse {
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
