mod data;

pub use data::fs_operator;
use opendal::Operator;
use surrealdb::sql::Id;

use crate::{backend::NotsBackend, utils::Secret};
use color_eyre::eyre::Result;
use hyper::Client;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{atomic::AtomicBool, Arc, RwLock},
};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Worker {
    pub app_id: String,
    pub updated_at: time::OffsetDateTime,

    pub container_id: Option<String>,
    pub process_id: Option<u32>,

    pub app_version: String,
}

pub type AppState = Arc<AppStateInner>;

#[derive(Clone, Serialize, Deserialize, Debug)]
struct Node {
    workers: HashMap<String, Worker>,
}

pub async fn try_new(
    db: crate::Database,
    file: Operator,
    kw_secret: &str,
    processes: Box<dyn NotsBackend>,
) -> Result<AppState> {
    if kw_secret.len() < 16 {
        panic!("kw_secret must be at least 16 characters long");
    }

    let file = data::Fs(file);
    let node_id = "1";

    let res = db
        .query("CREATE node:$id SET $data")
        .bind(("id", node_id))
        .bind((
            "data",
            Node {
                workers: HashMap::new(),
            },
        ))
        .await?;

    // let res: Option<Node> = db.create(("nodes", Id::from("node1"))).await?;

    // let workers: HashMap<String, Worker> =
    //     db.select(("workers", node_id)).await?.unwrap_or_default();

    Ok(AppStateInner {
        stated_at: time::OffsetDateTime::now_utc(),
        db,
        file,
        kw_secret: Secret::new(kw_secret.to_string()),
        running: AtomicBool::new(false),
        processes,
        client: Client::default(),
        workers: RwLock::new(HashMap::new()),
    }
    .into())
}

pub struct AppStateInner {
    pub running: AtomicBool,
    pub stated_at: time::OffsetDateTime,

    pub file: data::Fs,      // files
    pub db: crate::Database, // possibly shared with other nots instances

    pub processes: Box<dyn NotsBackend>,

    pub kw_secret: Secret,
    pub client: Client<hyper::client::HttpConnector>,

    pub workers: RwLock<HashMap<String, Worker>>,
}

impl AppStateInner {
    pub async fn run(&self) -> Result<()> {
        use std::sync::atomic::Ordering::Relaxed;
        if self.running.load(Relaxed) {
            panic!("State is already running");
        }

        self.running.store(true, Relaxed);
        loop {
            // if let Err(e) = self.update_apps().await {
            //     warn!("Failed to update apps: {}", e);
            // }

            tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;

            // if let Err(e) = self.persist_workers().await {
            //     warn!("Failed to persist workers: {}", e);
            // }
        }
    }

    pub(crate) async fn get_proxy_uri(&self, uri: hyper::Uri) -> hyper::Uri {
        let mut new_uri_parts = hyper::http::uri::Parts::default();
        new_uri_parts.scheme = Some("http".parse().unwrap());
        new_uri_parts.authority = Some("localhost:3333".parse().unwrap());
        new_uri_parts.path_and_query = uri.path_and_query().cloned();
        hyper::Uri::from_parts(new_uri_parts).unwrap()
    }

    // pub async fn create_app(&self, app: App) -> Result<()> {
    //     let appid = cuid2::create_id();
    //     appids.push(appid.clone());
    //     self.db.write("apps", &appids).await?;
    //     self.db.write(&format!("apps/{}", appid), &app).await?;
    //     self.db
    //         .write("apps_updated_at", &time::OffsetDateTime::now_utc())
    //         .await?;
    //     Ok(())
    // }

    // async fn get_apps(&self) -> Result<HashMap<String, App>> {
    //     let appids = self.db.read::<Vec<String>>("apps").await?;
    //     let mut apps = HashMap::new();

    //     for appid in appids.iter() {
    //         if self.db.stat(&format!("apps/{}", appid)).await?.is_none() {
    //             tracing::error!("App {} is missing from database", appid);
    //         } else {
    //             let app = self.db.read::<App>(&format!("apps/{}", appid)).await?;
    //             apps.insert(appid.clone(), app);
    //         }
    //     }

    //     Ok(apps)
    // }

    // async fn persist_workers(&self) -> Result<()> {
    //     let workers = self.workers.read().expect("worker lock poisoned").clone();
    //     self.local.write("workers", &workers).await?;
    //     Ok(())
    // }
}
