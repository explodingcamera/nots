use std::net::SocketAddr;

use axum::{
    extract::{ConnectInfo, State},
    Router,
};
use hyper::{Body, Request, Response, Uri};

use crate::{error::Error, state::AppState, utils};

pub fn new(app_state: AppState) -> Router {
    Router::new().fallback(handler).with_state(app_state)
}

pub async fn handler(
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    mut req: Request<Body>,
) -> Result<Response<Body>, Error> {
    let mut new_uri_parts = hyper::http::uri::Parts::default();
    new_uri_parts.scheme = Some("http".parse().unwrap());
    new_uri_parts.authority = Some("localhost:3333".parse().unwrap());
    new_uri_parts.path_and_query = req.uri().path_and_query().cloned();
    let new_uri = Uri::from_parts(new_uri_parts).unwrap();

    utils::add_x_forwarded_for(req.headers_mut(), addr);
    *req.uri_mut() = new_uri;

    let Ok(mut res) = state.client.request(req).await else {
        return Err(Error("Could not proxy request".to_string(), 500));
    };

    utils::remove_hop_by_hop_headers(res.headers_mut());
    Ok(res)
}
