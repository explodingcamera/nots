#![allow(dead_code)]
#![allow(unused)]

mod code;
mod error;
mod http;
mod process;
mod state;
mod utils;

use crate::process::{DockerBackend, DockerBackendSettings};
use state::AppState;
use std::{env, net::SocketAddr, path::PathBuf};
use tracing::info;

static DEV: bool = cfg!(debug_assertions);

#[tokio::main]
async fn main() -> color_eyre::eyre::Result<()> {
    nots_client::install_tracing(None);
    color_eyre::install()?;

    let mut secret = env::var("NOTS_SECRET").unwrap_or_default();
    if secret.is_empty() {
        match DEV {
            true => secret = "00000000000000000000000000000000".to_string(),
            false => panic!("NOTS_SECRET must be set"),
        }
    }

    tokio::fs::create_dir_all("data/fs").await?;
    let fs = state::fs_operator("data/fs")?;
    let kv = state::persy_operator("data/kv.persy")?;
    let backend: Box<dyn process::ProcessBackend + Sync> = match env::var("NOTS_BACKEND")
        .unwrap_or("docker".to_string())
        .as_str()
    {
        "docker" => Box::new(DockerBackend::try_new(DockerBackendSettings::default())?),
        backend => panic!("Unknown backend: {}", backend),
    };

    let app_state = AppState::new(kv, fs, secret, backend);

    let reverse_proxy_addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let reverse_proxy = axum::Server::bind(&reverse_proxy_addr).serve(
        http::proxy::new(app_state.clone()).into_make_service_with_connect_info::<SocketAddr>(),
    );

    let worker_socket_path = PathBuf::from("/tmp/nots/worker.sock");
    let worker_socket = utils::create_unix_socket(worker_socket_path.clone()).await;
    let worker_api = axum::Server::builder(worker_socket).serve(
        http::worker::new(app_state.clone())
            .into_make_service_with_connect_info::<crate::utils::UdsConnectInfo>(),
    );

    let api_socket_path = PathBuf::from("/tmp/nots/api.sock");
    let api_socket = utils::create_unix_socket(api_socket_path.clone()).await;
    let api = axum::Server::builder(api_socket).serve(
        http::api::new(app_state.clone())
            .into_make_service_with_connect_info::<crate::utils::UdsConnectInfo>(),
    );

    info!("Gateway listening on {}", reverse_proxy_addr);
    info!("Worker API listening on {}", worker_socket_path.display());
    info!("API listening on {}", api_socket_path.display());

    let scheduler = app_state.run();

    tokio::select! {
        res = reverse_proxy => res?,
        res = worker_api => res?,
        res = api => res?,
        res = scheduler => res?,
    };

    info!("Shutting down");
    Ok(())
}
