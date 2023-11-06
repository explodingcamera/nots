mod data;

pub use data::{fs_operator, persy_operator};
use nots_client::models::App;
use tracing::{info, warn};

use crate::{runtime::NotsRuntime, utils::Secret};
use color_eyre::eyre::Result;
use hyper::Client;
use opendal::Operator;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{atomic::AtomicBool, Arc, RwLock},
};

#[derive(Clone, Serialize, Deserialize)]
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

    pub file: data::Fs,  // files
    pub db: data::Kv,    // possibly shared with other nots instances
    pub local: data::Kv, // local state

    pub processes: Arc<Box<dyn NotsRuntime>>,

    pub kw_secret: Secret,
    pub client: Client<hyper::client::HttpConnector>,

    pub apps: Arc<RwLock<HashMap<String, App>>>,
    pub apps_updated_at: Arc<RwLock<time::OffsetDateTime>>,

    pub workers: Arc<RwLock<HashMap<String, Worker>>>,
}

impl AppState {
    pub async fn try_new(
        db: Operator,
        local: Operator,
        file: Operator,
        kw_secret: String,
        processes: Box<dyn NotsRuntime>,
    ) -> Result<Self> {
        if kw_secret.len() < 32 {
            panic!("kw_secret must be at least 32 characters long");
        }
        let file = data::Fs(file);
        let db = data::Kv(db);
        let local = data::Kv(local);

        if db.stat("apps_updated_at").await?.is_none() {
            db.write("apps_updated_at", &time::OffsetDateTime::now_utc())
                .await?;
        }

        let workers = {
            if local.stat("workers").await?.is_none() {
                let workers: HashMap<String, Worker> = HashMap::new();
                local.write("workers", &workers).await?;
            }
            local.read::<HashMap<String, Worker>>("workers").await?
        };
        info!("Loaded {} workers", workers.len());

        Ok(Self {
            db,
            local,
            file,
            kw_secret: Secret::new(kw_secret),
            running: Arc::new(AtomicBool::new(false)),
            processes: Arc::new(processes),
            client: Client::default(),
            workers: Arc::new(RwLock::new(workers)),
            apps: Arc::new(RwLock::new(HashMap::new())),
            apps_updated_at: Arc::new(RwLock::new(time::OffsetDateTime::UNIX_EPOCH)),
        })
    }

    pub async fn run(&self) -> Result<()> {
        use std::sync::atomic::Ordering::Relaxed;
        if self.running.load(Relaxed) {
            panic!("State is already running");
        }

        self.running.store(true, Relaxed);
        loop {
            if let Err(e) = self.update_apps().await {
                warn!("Failed to update apps: {}", e);
            }

            tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;

            if let Err(e) = self.persist_workers().await {
                warn!("Failed to persist workers: {}", e);
            }
        }
    }

    async fn update_apps(&self) -> Result<()> {
        let apps_updated_at = self
            .db
            .read::<time::OffsetDateTime>("apps_updated_at")
            .await?;

        let self_updated_at = *self
            .apps_updated_at
            .read()
            .expect("apps_updated_at lock poisoned");

        if apps_updated_at < self_updated_at {
            return Ok(());
        }

        let apps = self.get_apps().await?;

        {
            *self
                .apps_updated_at
                .write()
                .expect("apps updated lock poisoned") = time::OffsetDateTime::now_utc();
            *self.apps.write().expect("apps lock poisoned") = apps;
        }

        Ok(())
    }

    async fn get_apps(&self) -> Result<HashMap<String, App>> {
        let appids = self.local.read::<Vec<String>>("apps").await?;
        let mut apps = HashMap::new();

        for appid in appids.iter() {
            if self.local.stat(&format!("apps/{}", appid)).await?.is_none() {
                tracing::error!("App {} is missing from database", appid);
            } else {
                let app = self.local.read::<App>(&format!("apps/{}", appid)).await?;
                apps.insert(appid.clone(), app);
            }
        }

        Ok(apps)
    }

    async fn persist_workers(&self) -> Result<()> {
        self.local.write("workers", &self.workers.as_ref()).await?;
        Ok(())
    }
}
