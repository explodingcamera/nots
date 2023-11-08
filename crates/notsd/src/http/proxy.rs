use std::net::SocketAddr;

use axum::{
    extract::{ConnectInfo, State},
    Router,
};
use hyper::{Body, Request, Response};

use crate::{error::Error, state::AppState, utils};

pub fn new(app_state: AppState) -> Router {
    Router::new().fallback(handler).with_state(app_state)
}

pub async fn handler(
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    mut req: Request<Body>,
) -> Result<Response<Body>, Error> {
    utils::add_x_forwarded_for(req.headers_mut(), addr);
    *req.uri_mut() = state.get_proxy_uri(req.uri().clone()).await;

    let Ok(mut res) = state.client.request(req).await else {
        return Err(Error("Could not proxy request".to_string(), 500));
    };

    utils::remove_hop_by_hop_headers(res.headers_mut());
    Ok(res)
}
