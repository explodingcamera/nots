use std::{net::SocketAddr, path::PathBuf};

use color_eyre::eyre::Result;

use crate::{state, utils};

pub(crate) mod api;
pub(crate) mod proxy;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use hyper::Body;
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
    let reverse_proxy_addr = reverse_proxy_addr.parse::<SocketAddr>()?;
    let reverse_proxy = axum::Server::bind(&reverse_proxy_addr)
        .serve(proxy::new(app_state.clone()).into_make_service_with_connect_info::<SocketAddr>());
    reverse_proxy.await?;
    Ok(())
}

pub async fn create_api(api_bind: &str, app_state: state::AppState) -> Result<()> {
    if api_bind.starts_with('/') && !api_bind.contains(':') {
        let api_socket_path = PathBuf::from(api_bind);
        let api_socket = utils::create_unix_socket(api_socket_path.clone()).await?;
        let api = axum::Server::builder(api_socket)
            .serve(api::new(app_state.clone()).into_make_service_with_connect_info::<crate::utils::UdsConnectInfo>());
        api.await
    } else {
        let api_socket_addr = api_bind.parse::<SocketAddr>()?;
        let api = axum::Server::bind(&api_socket_addr)
            .serve(api::new(app_state.clone()).into_make_service_with_connect_info::<SocketAddr>());
        api.await
    };

    Ok(())
}
