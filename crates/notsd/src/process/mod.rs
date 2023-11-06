use std::collections::HashMap;

use color_eyre::eyre::Result;
use nots_client::api::App;
use tokio_cron_scheduler::{Job, JobScheduler};

use crate::state::AppState;

#[cfg(feature = "docker")]
mod docker;

#[cfg(feature = "docker")]
pub use docker::{DockerBackend, DockerBackendSettings};

#[cfg(feature = "systemd")]
mod systemd;

pub struct CreateWorker {}

pub trait ProcessBackend: Send + Sync {
    fn worker_create(&self, worker: CreateWorker) -> Result<()>;
    fn workers_get(&self) -> Result<()>;
    fn worker_get(&self, id: &str) -> Result<()>;
    fn worker_remove(&self, id: &str) -> Result<()>;
    fn worker_update(&self) -> Result<()>;
    fn worker_restart(&self, id: &str) -> Result<()>;
    fn worker_status(&self, id: &str) -> Result<()>;
}
