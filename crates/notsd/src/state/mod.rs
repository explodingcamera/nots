mod data;

pub use data::{fs_operator, persy_operator};
use nots_client::models::App;

use crate::{runtime::NotsRuntime, utils::Secret};
use color_eyre::eyre::{Context, Result};
use dashmap::DashMap;
use hyper::Client;
use opendal::Operator;
use std::sync::{atomic::AtomicBool, Arc};
use tokio_cron_scheduler::{Job, JobScheduler};

#[derive(Clone)]
pub struct Worker {
    pub app_id: String,
    pub updated_at: time::OffsetDateTime,

    pub container_id: Option<String>,
    pub process_id: Option<u32>,

    pub app_version: String,
}

#[derive(Clone)]
pub struct AppState {
    running: Arc<AtomicBool>,

    pub kv: data::Kv,
    pub file: data::Fs,

    pub processes: Arc<Box<dyn NotsRuntime>>,

    pub kw_secret: Arc<Secret>,
    pub client: Client<hyper::client::HttpConnector>,

    pub apps: DashMap<String, App>,
    pub workers: DashMap<String, Worker>,
}

impl AppState {
    pub fn new(
        kv: Operator,
        file: Operator,
        kw_secret: String,
        processes: Box<dyn NotsRuntime>,
    ) -> Self {
        if kw_secret.len() < 32 {
            panic!("kw_secret must be at least 32 characters long");
        }

        Self {
            running: Arc::new(AtomicBool::new(false)),
            file: data::Fs(file),
            kv: data::Kv(kv),
            kw_secret: Arc::new(Secret::new(kw_secret)),
            processes: Arc::new(processes),
            client: Client::default(),

            apps: DashMap::new(),
            workers: DashMap::new(),
        }
    }

    pub async fn run(&self) -> Result<()> {
        use std::sync::atomic::Ordering::Relaxed;
        if self.running.load(Relaxed) {
            panic!("State is already running");
        }
        self.running.store(true, Relaxed);

        let mut sched = JobScheduler::new().await?;

        sched
            .add(Job::new("1/10 * * * * *", |_uuid, _l| {
                println!("Tick");
            })?)
            .await?;

        sched.start().await?;
        std::future::pending().await
    }
}
