mod data;
mod error;
mod proxy;
mod state;
mod utils;

use axum::{routing::any, Router};
use state::AppState;
use std::{env, net::SocketAddr};
use tracing::info;

static DEV: bool = cfg!(debug_assertions);

#[tokio::main]
async fn main() -> color_eyre::eyre::Result<()> {
    utils::install_tracing();
    color_eyre::install()?;

    let mut secret = env::var("NOTS_SECRET").unwrap_or_default();
    if secret.is_empty() {
        match DEV {
            true => secret = "Ef7upsA8Pd9zPf49NWLeKRaGaSYkmGDc".to_string(),
            false => panic!("NOTS_SECRET must be set"),
        }
    }

    let data = data::Data::new_with_persy("data.persy")?;
    let app_state = AppState::new(data, secret);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let app = Router::new()
        .route("/", any(proxy::handler))
        .with_state(app_state);
    let server =
        axum::Server::bind(&addr).serve(app.into_make_service_with_connect_info::<SocketAddr>());

    info!("Server running on http://{}", addr);

    Ok(server.await?)
}
