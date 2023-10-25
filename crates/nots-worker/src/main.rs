use std::sync::{Arc, RwLock};

use color_eyre::eyre::Result;
use futures_retry::{FutureRetry, RetryPolicy};
use hyper::{Body, Response};
use hyperlocal::UnixConnector;
use nots_core::worker::{WorkerHeartbeatResponse, WorkerRegisterResponse, WorkerSettings};
use tracing::{debug, error, info};

mod utils;
const SOCKET_PATH: &str = "/tmp/nots/worker.sock";

struct State {
    pub settings: RwLock<Option<WorkerSettings>>,
    pub client: hyper::Client<UnixConnector>,
    pub worker_id: String,
}

impl State {
    async fn req(&self, uri: &str, method: &str, body: Option<Body>) -> Result<Response<Body>> {
        let addr: hyper::Uri = hyperlocal::Uri::new(SOCKET_PATH, uri).into();
        debug!("req: {}", addr);

        let req = hyper::Request::builder()
            .method(method)
            .header("x-nots-worker-id", &self.worker_id)
            .header("x-nots-worker-version", env!("CARGO_PKG_VERSION"))
            .uri(addr)
            .body(body.unwrap_or(Body::empty()))?;

        let res = self.client.request(req).await?;
        Ok(res)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    nots_core::install_tracing(None);

    let worker_id =
        std::env::var("NOTS_WORKER_ID").unwrap_or_else(|_| match cfg!(debug_assertions) {
            true => "test".to_owned(),
            false => panic!("NOTS_WORKER_ID needs to be set"),
        });

    let state = State {
        settings: RwLock::new(None),
        client: hyper::Client::builder().build(UnixConnector),
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

    loop {
        heartbeat(state.clone()).await?;
        tokio::time::sleep(tokio::time::Duration::from_secs(20)).await;
    }
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

async fn heartbeat(state: Arc<State>) -> Result<()> {
    let res = state.req("/worker/heartbeat", "POST", None).await?;
    let res = utils::parse_body::<WorkerHeartbeatResponse>(res).await?;
    debug!("heartbeat: {:?}", res);
    Ok(())
}
