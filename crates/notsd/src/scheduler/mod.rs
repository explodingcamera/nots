use color_eyre::eyre::Result;
use nots_core::app::AppSettings;
use tokio_cron_scheduler::{Job, JobScheduler};

use crate::state::AppState;

mod container;

#[cfg(feature = "systemd")]
mod systemd;

pub struct Scheduler {
    state: AppState,
}

impl Scheduler {
    pub fn new(state: AppState) -> Self {
        Self { state }
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
