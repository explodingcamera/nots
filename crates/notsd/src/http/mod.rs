use std::net::SocketAddr;

use color_eyre::eyre::Result;
use tokio::net::TcpListener;

use crate::state;

pub(crate) mod api;
pub(crate) mod proxy;

use axum::{
    body::Body,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

pub struct Error(pub String, pub u16);
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let status = StatusCode::from_u16(self.1).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        let error_json = json!({ "error": self.0 }).to_string();
        let response = Response::builder()
            .status(status)
            .header("Content-Type", "application/json")
            .body(Body::from(error_json))
            .unwrap();

        response.into_response()
    }
}

pub async fn create_reverse_proxy(reverse_proxy_addr: &str, app_state: state::AppState) -> Result<()> {
    let listerner = TcpListener::bind(reverse_proxy_addr).await?;

    let reverse_proxy = axum::serve(
        listerner,
        proxy::new(app_state.clone()).into_make_service_with_connect_info::<SocketAddr>(),
    );

    reverse_proxy.await?;
    Ok(())
}

pub async fn create_api(api_addr: &str, app_state: state::AppState) -> Result<()> {
    let listener = TcpListener::bind(api_addr).await?;

    let api = axum::serve(
        listener,
        api::new(app_state.clone()).into_make_service_with_connect_info::<SocketAddr>(),
    );

    api.await;
    Ok(())
}
