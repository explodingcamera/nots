mod data;
mod kw_secret;

pub use data::{fs_operator, persy_operator};

use self::kw_secret::KWSecret;
use crate::process::ProcessBackend;
use aes_kw::KekAes256;
use color_eyre::eyre::{Context, Result};
use dashmap::DashMap;
use hyper::Client;
use nots_client::{api::App, EncryptedBytes};
use opendal::Operator;
use std::sync::{atomic::AtomicBool, Arc};
use tokio_cron_scheduler::{Job, JobScheduler};
use zeroize::{Zeroize, ZeroizeOnDrop, Zeroizing};

#[derive(Clone)]
pub struct AppInstance {
    pub app_id: String,
    pub updated_at: time::OffsetDateTime,

    pub container_id: Option<String>,
    pub process_id: Option<u32>,
}

#[derive(Clone)]
pub struct AppState {
    running: Arc<AtomicBool>,

    pub kv: data::kv::Operator,
    pub file: data::file::Operator,

    pub processes: Arc<Box<dyn ProcessBackend>>,

    pub kw_secret: Arc<KWSecret>,
    pub client: Client<hyper::client::HttpConnector>,

    pub apps: DashMap<String, App>,
    pub app_instances: DashMap<String, AppInstance>,
}

impl AppState {
    pub fn new(
        kv: Operator,
        file: Operator,
        kw_secret: String,
        processes: Box<dyn ProcessBackend>,
    ) -> Self {
        if kw_secret.len() < 32 {
            panic!("kw_secret must be at least 32 characters long");
        }

        Self {
            running: Arc::new(AtomicBool::new(false)),
            file: data::file::Operator(file),
            kv: data::kv::Operator(kv),
            kw_secret: Arc::new(KWSecret::new(kw_secret)),
            processes: Arc::new(processes),
            client: Client::default(),

            apps: DashMap::new(),
            app_instances: DashMap::new(),
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
