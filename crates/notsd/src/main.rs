#![allow(dead_code)]
#![allow(unused)]

mod code;
mod data;
mod error;
mod git;
mod http;
mod scheduler;
mod state;
mod utils;

use state::AppState;
use std::{env, net::SocketAddr, path::PathBuf};
use tracing::info;

static DEV: bool = cfg!(debug_assertions);

#[tokio::main]
async fn main() -> color_eyre::eyre::Result<()> {
    nots_core::install_tracing(None);
    color_eyre::install()?;

    crate::git::clone(
        "https://github.com/explodingcamera/esm",
        None,
        None,
        "/tmp/nots/test2",
    )
    .await?;

    let mut secret = env::var("NOTS_SECRET").unwrap_or_default();
    if secret.is_empty() {
        match DEV {
            true => secret = "Ef7upsA8Pd9zPf49NWLeKRaGaSYkmGDc".to_string(),
            false => panic!("NOTS_SECRET must be set"),
        }
    }

    let data = data::Data::new_with_persy("data.persy")?;
    let app_state = AppState::new(data, secret);

    let reverse_proxy_addr = SocketAddr::from(([127, 0, 0, 1], 4100));
    let reverse_proxy = axum::Server::bind(&reverse_proxy_addr).serve(
        http::proxy::new(app_state.clone()).into_make_service_with_connect_info::<SocketAddr>(),
    );

    let worker_socket = utils::create_unix_socket(PathBuf::from("/tmp/nots/worker.sock")).await;
    let worker_api = axum::Server::builder(worker_socket).serve(
        http::worker::new(app_state.clone())
            .into_make_service_with_connect_info::<crate::utils::UdsConnectInfo>(),
    );

    let api_addr = SocketAddr::from(([127, 0, 0, 1], 4101));
    let api = axum::Server::bind(&api_addr).serve(
        http::api::new(app_state.clone()).into_make_service_with_connect_info::<SocketAddr>(),
    );

    info!("Gateway listening on {}", reverse_proxy_addr);
    info!("Worker API listening on /tmp/nots/worker.sock");
    info!("API listening on {}", api_addr);

    let _scheduler = scheduler::Scheduler::new(app_state);

    tokio::select! {
        res = reverse_proxy => res?,
        res = worker_api => res?,
        res = api => res?,
    };

    Ok(())
}
