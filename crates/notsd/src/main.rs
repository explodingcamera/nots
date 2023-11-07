#![allow(dead_code)]
#![allow(unused)]
#![warn(unused_imports)]

mod code;
mod error;
mod http;
mod runtime;
mod state;
mod utils;

use crate::runtime::{DockerBackendSettings, DockerRuntime};
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
    let db = state::persy_operator("data/kv.persy")?;
    let local = state::persy_operator("data/local.persy")?;

    let backend: Box<dyn runtime::NotsRuntime + Sync> = match env::var("NOTS_BACKEND")
        .unwrap_or("docker".to_string())
        .as_str()
    {
        "docker" => Box::new(DockerRuntime::try_new(DockerBackendSettings::default())?),
        backend => panic!("Unknown backend: {}", backend),
    };

    let app_state = state::try_new(db, local, fs, secret, backend).await?;

    let reverse_proxy_addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let reverse_proxy = axum::Server::bind(&reverse_proxy_addr).serve(
        http::proxy::new(app_state.clone()).into_make_service_with_connect_info::<SocketAddr>(),
    );

    let api_socket_path = PathBuf::from("/tmp/nots/api.sock");
    let api_socket = utils::create_unix_socket(api_socket_path.clone()).await;
    let api = axum::Server::builder(api_socket).serve(
        http::api::new(app_state.clone())
            .into_make_service_with_connect_info::<crate::utils::UdsConnectInfo>(),
    );

    info!("Gateway listening on {}", reverse_proxy_addr);
    info!("API listening on {}", api_socket_path.display());

    let scheduler = app_state.run();

    tokio::select! {
        res = reverse_proxy => res?,
        res = api => res?,
        res = scheduler => res?,
    };

    info!("Shutting down");
    Ok(())
}
