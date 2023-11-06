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

#[async_trait]
pub trait NotsRuntime: Send + Sync {
    async fn workers_get(&self) -> Result<HashMap<String, WorkerStatus>>;
    async fn worker_create(&self, worker: CreateWorker) -> Result<()>;
    async fn worker_state(&self, id: &str) -> Result<WorkerState>;
    async fn worker_remove(&self, id: &str) -> Result<()>;
}

pub struct CreateWorker {
    pub worker_id: String,
    pub runtime_options: WorkerRuntimeOptions,
}
