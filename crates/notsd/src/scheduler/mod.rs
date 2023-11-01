use color_eyre::eyre::Result;
use nots_core::app::AppSettings;
use tokio_cron_scheduler::{Job, JobScheduler};

use crate::state::AppState;

#[cfg(feature = "docker")]
mod container;
#[cfg(feature = "docker")]
pub use container::docker::{DockerBackend, DockerBackendSettings};

#[cfg(feature = "systemd")]
mod systemd;

pub struct CreateWorker {}

pub trait ProcessBackend {
    fn worker_create(&self, worker: CreateWorker) -> Result<()>;
    fn workers_get(&self) -> Result<()>;
    fn worker_get(&self, id: &str) -> Result<()>;
    fn worker_remove(&self, id: &str) -> Result<()>;
    fn worker_update(&self) -> Result<()>;
    fn worker_restart(&self, id: &str) -> Result<()>;
    fn worker_status(&self, id: &str) -> Result<()>;
}

pub struct Scheduler {
    state: AppState,
    backend: Box<dyn ProcessBackend + Sync>,
}

impl Scheduler {
    pub fn new(state: AppState, backend: Box<dyn ProcessBackend + Sync>) -> Self {
        Self { state, backend }
    }

    pub async fn run(&self) -> Result<()> {
        let mut sched = JobScheduler::new().await?;

        sched
            .add(Job::new("1/10 * * * * *", |_uuid, _l| {
                println!("I run every 10 seconds");
            })?)
            .await?;

        sched.shutdown_on_ctrl_c();
        sched.set_shutdown_handler(Box::new(|| {
            Box::pin(async move {
                println!("Shut down done");
            })
        }));

        sched.start().await?;
        std::future::pending().await
    }
}

pub struct RunningApp {
    pub settings: AppSettings,
    pub settings_updated_at: time::OffsetDateTime,

    pub container_id: Option<String>,
    pub process_id: Option<u32>,
}
