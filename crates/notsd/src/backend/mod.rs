use axum::async_trait;
use color_eyre::eyre::Result;
use nots_client::models::{WorkerRuntimeOptions, WorkerState, WorkerStatus};
use std::collections::HashMap;

#[cfg(feature = "docker")]
mod docker;

#[cfg(feature = "docker")]
pub use docker::*;

#[cfg(feature = "process")]
mod process;

pub fn try_new(backend: &str) -> Result<Box<dyn NotsBackend + Sync>> {
    match backend {
        #[cfg(feature = "docker")]
        "docker" => Ok(Box::new(DockerRuntime::try_new(
            DockerBackendSettings::default(),
        )?)),
        #[cfg(feature = "process")]
        "process" => Ok(Box::new(process::ProcessRuntime::new())),
        backend => panic!("Unknown backend: {}", backend),
    }
}

#[async_trait]
pub trait NotsBackend: Send + Sync {
    async fn workers_get(&self) -> Result<HashMap<String, WorkerStatus>>;
    async fn worker_create(&self, worker: CreateWorker) -> Result<()>;
    async fn worker_state(&self, id: &str) -> Result<WorkerState>;
    async fn worker_remove(&self, id: &str) -> Result<()>;
}

pub struct CreateWorker {
    pub worker_id: String,
    pub runtime_options: WorkerRuntimeOptions,
}
