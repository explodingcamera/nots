use std::{
    path::Path,
    sync::{Arc, RwLock},
};

use color_eyre::eyre::Result;
use futures_retry::{FutureRetry, RetryPolicy};
use hyper::{body::HttpBody, Body, Response};
use nots_client::{models::WorkerSettings, worker::*};
use tokio::io::AsyncWriteExt;
use tracing::{error, info};

mod utils;
const SOCKET_PATH: &str = "/tmp/nots/worker.sock";
const SOURCE_VERSION_PATH: &str = "~/nots/source-version";

struct State {
    pub settings: RwLock<Option<WorkerSettings>>,
    pub client: nots_client::Client,
    pub worker_id: String,
}

impl State {
    async fn req(&self, uri: &str, method: &str, body: Option<Body>) -> Result<Response<Body>> {
        let mut headers = hyper::HeaderMap::new();
        headers.insert("x-nots-worker-id", self.worker_id.parse()?);
        headers.insert("x-nots-worker-version", env!("CARGO_PKG_VERSION").parse()?);
        let res = self.client.req(uri, method, body, Some(headers)).await?;
        Ok(res)
    }
}

pub async fn current_app_version() -> Result<Option<String>> {
    let path = Path::new(SOURCE_VERSION_PATH);
    if !path.try_exists().unwrap_or(false) {
        return Ok(None);
    }

    let version = tokio::fs::read_to_string(path).await?;
    Ok(Some(version))
}

pub async fn set_current_app_version(version: &str) -> Result<()> {
    let path = std::path::Path::new(SOURCE_VERSION_PATH);
    tokio::fs::write(path, version).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    nots_client::install_tracing(None);

    let worker_id =
        std::env::var("NOTS_WORKER_ID").unwrap_or_else(|_| match cfg!(debug_assertions) {
            true => "test".to_owned(),
            false => panic!("NOTS_WORKER_ID needs to be set"),
        });

    let state = State {
        settings: RwLock::new(None),
        client: nots_client::Client::unix(SOCKET_PATH.into()),
        worker_id,
    };

    let state = Arc::new(state);
    let mut tries = 0;

    FutureRetry::new(
        || register(state.clone()),
        |e: color_eyre::eyre::Error| {
            error!("err: {}", e);
            if tries > 5 {
                return RetryPolicy::ForwardError(color_eyre::eyre::eyre!(
                    "failed to register worker"
                ));
            }
            tries += 1;
            RetryPolicy::WaitRetry::<color_eyre::eyre::Error>(std::time::Duration::from_secs(1))
        },
    )
    .await
    .map_err(|_| color_eyre::eyre::eyre!("failed to register worker"))?;

    update_source_if_needed(state.clone()).await?;

    unimplemented!();
}

async fn register(state: Arc<State>) -> Result<()> {
    let res = state.req("/worker/register", "POST", None).await?;
    let body = utils::parse_body::<WorkerRegisterResponse>(res).await?;

    state
        .settings
        .write()
        .expect("failed to lock settings")
        .replace(body.settings);

    info!("registered worker: {:?}", state.worker_id);

    Ok(())
}

async fn update_source_if_needed(state: Arc<State>) -> Result<()> {
    let current_version = current_app_version().await?;

    let res = state
        .req(
            "/worker/source",
            "GET",
            Some(Body::from(serde_json::to_string(&WorkerSourceRequest {
                source_version: current_version.clone(),
            })?)),
        )
        .await?;

    let file_version = res
        .headers()
        .get("x-nots-worker-source-version")
        .and_then(|v| v.to_str().ok())
        .map(|v| v.to_owned())
        .ok_or_else(|| color_eyre::eyre::eyre!("failed to get source version"))?;

    if current_version.is_some_and(|v| v == file_version) {
        info!("source is up to date");
        return Ok(());
    }

    let path = &format!("/tmp/nots/{}.tar.gz", file_version);
    let path = Path::new(path);

    tokio::fs::create_dir_all(path).await?;
    let mut file = tokio::fs::File::create(path).await?;

    let mut body = res.into_body();
    while let Some(chunk) = body.data().await {
        file.write_all(&chunk?).await?;
    }

    unimplemented!();
}
