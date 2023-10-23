mod data;
mod error;
mod utils;

use axum::{extract::ConnectInfo, routing::any, Router};
use error::Error;
use hyper::{Body, Client, Request, Response, Uri};
use std::net::SocketAddr;

#[axum::debug_handler]
async fn proxy(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    req: Request<Body>,
) -> Result<Response<Body>, Error> {
    let client = Client::new();

    let uri_string = format!(
        "http://localhost:3333{}",
        req.uri().path_and_query().unwrap().to_string()
    );
    let new_uri: Uri = uri_string.parse().unwrap();

    let (parts, body) = req.into_parts();

    let mut new_req = Request::builder()
        .method(parts.method)
        .uri(new_uri)
        .body(body)
        .unwrap();

    *new_req.headers_mut() = parts.headers.clone();
    utils::add_x_forwarded_for(new_req.headers_mut(), addr);

    let Ok(res) = client.request(new_req).await else {
        return Err(Error("Could not proxy request".to_string(), 500));
    };

    let (parts, body) = res.into_parts();

    let mut new_res = Response::new(body);
    *new_res.status_mut() = parts.status;
    *new_res.headers_mut() = parts.headers.clone();
    utils::remove_hop_by_hop_headers(new_res.headers_mut());

    Ok(new_res)
}

#[tokio::main]
async fn main() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;

    let data = data::Data::new_with_persy("data")?;

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let app = Router::new().route("/", any(proxy));
    let server =
        axum::Server::bind(&addr).serve(app.into_make_service_with_connect_info::<SocketAddr>());

    println!("Server running on http://{}", addr);

    Ok(server.await?)
}
