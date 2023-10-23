mod data;
mod error;
mod proxy;
mod state;
mod utils;

use axum::{routing::any, Router};
use state::AppState;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;

    let data = data::Data::new_with_persy("data.persy")?;
    let app_state = AppState::new(data, "oAWRD78jViYRJK9on9afTP@4nE$uRpu9".to_string());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let app = Router::new()
        .route("/", any(proxy::handler))
        .with_state(app_state);
    let server =
        axum::Server::bind(&addr).serve(app.into_make_service_with_connect_info::<SocketAddr>());

    println!("Server running on http://{}", addr);

    Ok(server.await?)
}
