use axum::body::StreamBody;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use color_eyre::eyre::Result;
use hyper::{header, HeaderMap};
use nots_core::worker::WorkerRegisterResponse;

use crate::state::AppState;

pub fn new(app_state: AppState) -> Router {
    Router::new()
        .route("/", get(hi))
        .route("/worker/register", post(register))
        .route("/worker/heartbeat", post(heartbeat))
        .route("/worker/source", get(source))
        .with_state(app_state)
}

#[axum::debug_handler]
async fn register() -> Json<WorkerRegisterResponse> {
    Json(WorkerRegisterResponse {
        settings: nots_core::worker::WorkerSettings {
            port: 4100,
            command: Option::None,
            main: Some("main.js".to_owned()),
            env: std::collections::HashMap::new(),
        },
    })
}

#[axum::debug_handler]
// return a tarball of the source code
async fn source() -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "text/plain".parse().unwrap());

    let body = StreamBody::default();
    (headers, body)
}

#[axum::debug_handler]
async fn heartbeat() -> Json<nots_core::worker::WorkerHeartbeatResponse> {
    Json(nots_core::worker::WorkerHeartbeatResponse { ok: true })
}

async fn hi() -> &'static str {
    "Hello, World!"
}
