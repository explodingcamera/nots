use axum::extract::State;
use axum::http::HeaderValue;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use hyper::Request;
use nots_client::api::{CreateAppRequest, ServerStatus};

use crate::state::AppState;

const POWERED_BY: &str = concat!("nots/", env!("CARGO_PKG_VERSION"));

async fn add_version<B>(request: Request<B>, next: Next<B>) -> Response {
    let mut response = next.run(request).await;
    response
        .headers_mut()
        .insert("x-powered-by", HeaderValue::from_static(POWERED_BY));
    response
}

pub fn new(app_state: AppState) -> Router {
    Router::new()
        .route("/", get(hi))
        .route("/status", get(server_status))
        .route("/app", post(create_app))
        .route("/app/:id", post(update_app))
        .route("/app/:id", get(get_app))
        .route("/apps", get(get_apps))
        .with_state(app_state)
        .layer(axum::middleware::from_fn(add_version))
}

async fn server_status(State(app): State<AppState>) -> Json<ServerStatus> {
    let uptime = time::OffsetDateTime::now_utc() - app.stated_at;

    Json(ServerStatus {
        version: env!("CARGO_PKG_VERSION").to_string(),
        uptime_secs: uptime.whole_seconds() as u64,
    })
}

async fn create_app(State(app): State<AppState>, body: Json<CreateAppRequest>) -> impl IntoResponse {
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
