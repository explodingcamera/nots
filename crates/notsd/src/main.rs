#![allow(dead_code)]
#![allow(unused)]
#![warn(unused_imports)]

mod backend;
mod code;
mod env;
mod http;
mod state;
mod utils;

use crate::http::*;
use color_eyre::eyre::Result;
use std::path::PathBuf;
use surrealdb::{engine::any::Any, Surreal};
use tracing::info;

pub(crate) type Database = Surreal<Any>;

#[tokio::main]
async fn main() -> Result<()> {
    nots_client::install_tracing(None);
    color_eyre::install()?;
    let env = crate::env::new();
    tokio::fs::create_dir_all("data/fs").await?;

    let backend = backend::try_new(&env.nots_backend)?;
    let app_state = state::try_new(
        create_db().await?,
        state::fs_operator("data/fs")?,
        &env.nots_secret,
        backend,
    )
    .await?;

    let reverse_proxy = create_reverse_proxy("127.0.0.1:8080", app_state.clone());
    let worker = create_worker(PathBuf::from(env.nots_worker_bind), app_state.clone());
    let api = create_api(&env.nots_api_bind, app_state.clone());

    info!("Gateway listening on 127.0.0.1:8080");
    info!("API listening on {}", env.nots_api_bind);
    let scheduler = app_state.run();

    tokio::select! {
        res = api => res?,
        res = worker => res?,
        res = scheduler => res?,
        res = reverse_proxy => res?,
    };

    info!("Shutting down");
    Ok(())
}

async fn create_db() -> Result<Database> {
    let shared: Database = Surreal::init();
    let cwd = std::env::current_dir()?;
    let path = cwd.join("data/data.db");
    shared
        .connect(format!("rocksdb://{}", path.to_str().unwrap()))
        .await?;
    shared.use_ns("nots").use_db("main").await?;
    Ok(shared)
}
