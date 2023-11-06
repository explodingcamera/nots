use axum::body::StreamBody;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use hyper::{header, HeaderMap};
use nots_client::worker::{
    WorkerReadyRequest, WorkerReadyResponse, WorkerRegisterResponse, WorkerSourceRequest,
};

use crate::state::AppState;

pub fn new(app_state: AppState) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/worker/ready", post(ready))
        .route("/worker/register", post(register))
        .route("/worker/source", get(source))
        .with_state(app_state)
}

#[axum::debug_handler]
async fn register() -> Json<WorkerRegisterResponse> {
    Json(WorkerRegisterResponse {
        settings: nots_client::models::WorkerSettings {
            port: Some(4100),
            command: None,
            main: Some("main.js".to_owned()),
            env: std::collections::HashMap::new(),
            prepare: None,
        },
    })
}

#[axum::debug_handler]
async fn ready(_body: Json<WorkerReadyRequest>) -> Json<WorkerReadyResponse> {
    Json(WorkerReadyResponse {})
}

#[axum::debug_handler]
// return a tarball of the source code
async fn source(body: Json<WorkerSourceRequest>) -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "application/tar".parse().unwrap());

    let body = StreamBody::default();
    (headers, body)
}

async fn root() -> &'static str {
    "Hi! This is the Nots worker API"
}
