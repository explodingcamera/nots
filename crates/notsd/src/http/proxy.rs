use super::Error;
use crate::state::AppState;
use axum::extract::{ConnectInfo, State};
use axum::http::HeaderValue;
use axum::Router;
use color_eyre::eyre::Result;
use hyper::{Body, HeaderMap, Request, Response};
use std::net::SocketAddr;

pub fn new(app_state: AppState) -> Router {
    Router::new().fallback(handler).with_state(app_state)
}

pub async fn handler(
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    mut req: Request<Body>,
) -> Result<Response<Body>, Error> {
    add_x_forwarded_for(req.headers_mut(), addr);
    *req.uri_mut() = state.get_proxy_uri(req.uri().clone()).await;

    let Ok(mut res) = state.client.request(req).await else {
        return Err(Error("Could not proxy request".to_string(), 500));
    };

    remove_hop_by_hop_headers(res.headers_mut());
    Ok(res)
}

fn add_x_forwarded_for(headers: &mut HeaderMap<HeaderValue>, addr: SocketAddr) {
    let client_ip = addr.ip().to_string();
    if let Some(existing_header) = headers.get("X-Forwarded-For") {
        // Append the client IP address if the header already exists
        let updated_header = format!("{}, {}", existing_header.to_str().unwrap_or(""), client_ip);
        headers.insert("X-Forwarded-For", updated_header.parse().unwrap());
    } else {
        // Add a new header if it doesn't already exist
        headers.insert("X-Forwarded-For", client_ip.parse().unwrap());
    }
}

fn remove_hop_by_hop_headers(headers: &mut HeaderMap<HeaderValue>) {
    headers.remove("connection");
    headers.remove("keep-alive");
    headers.remove("proxy-authenticate");
    headers.remove("proxy-authorization");
    headers.remove("te");
    headers.remove("trailers");
    headers.remove("transfer-encoding");
    headers.remove("upgrade");
}
