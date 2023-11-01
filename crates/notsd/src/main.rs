#![allow(dead_code)]
#![allow(unused)]

mod code;
mod data;
mod error;
// mod git;
mod http;
mod scheduler;
mod state;
mod utils;

use crate::scheduler::{DockerBackend, DockerBackendSettings};
use state::AppState;
use std::{env, net::SocketAddr, path::PathBuf};
use tracing::info;

static DEV: bool = cfg!(debug_assertions);

#[tokio::main]
async fn main() -> color_eyre::eyre::Result<()> {
    nots_core::install_tracing(None);
    color_eyre::install()?;

    let mut secret = env::var("NOTS_SECRET").unwrap_or_default();
    if secret.is_empty() {
        match DEV {
            true => secret = "00000000000000000000000000000000".to_string(),
            false => panic!("NOTS_SECRET must be set"),
        }
    }

    let data = data::Data::new_with_persy("data.persy")?;
    let app_state = AppState::new(data, secret);

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

    let backend = env::var("NOTS_BACKEND").unwrap_or("docker".to_string());
    let backend: Box<dyn scheduler::ProcessBackend + Sync> = match backend.as_str() {
        "docker" => Box::new(DockerBackend::try_new(
            app_state.clone(),
            DockerBackendSettings::default(),
        )?),
        _ => panic!("Unknown backend: {}", backend),
    };

    let scheduler = scheduler::Scheduler::new(app_state.clone(), backend);
    let scheduler = scheduler.run();

    tokio::select! {
        res = reverse_proxy => res?,
        res = worker_api => res?,
        res = scheduler => res?,
        res = api => res?,
    };

    info!("Shutting down");
    Ok(())
}
