mod data;

pub use data::fs_operator;
use nots_client::models::App;
use tracing::warn;

use crate::{backend::NotsBackend, utils::Secret};
use color_eyre::eyre::Result;
use hyper::Client;
use opendal::Operator;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{atomic::AtomicBool, Arc},
};

use heed::{
    types::{SerdeRmp, Str},
    Database, PutFlags, RoTxn, RwTxn,
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

pub async fn try_new(
    db_env: heed::Env,
    file: Operator,
    kw_secret: &str,
    processes: Box<dyn NotsBackend>,
) -> Result<AppState> {
    if kw_secret.len() < 16 {
        panic!("kw_secret must be at least 16 characters long");
    }

    let file = data::Fs(file);

    let mut wtxn = db_env.write_txn()?;
    let apps = db_env.create_database(&mut wtxn, Some("apps"))?;
    let workers = db_env.create_database(&mut wtxn, Some("workers"))?;
    wtxn.commit()?;

    Ok(AppStateInner {
        db_env,
        apps,
        workers,
        stated_at: time::OffsetDateTime::now_utc(),
        file,
        kw_secret: Secret::new(kw_secret.to_string()),
        running: AtomicBool::new(false),
        processes,
        client: Client::default(),
    }
    .into())
}

pub struct AppStateInner {
    pub db_env: heed::Env,
    pub apps: Database<Str, SerdeRmp<App>>,
    pub workers: Database<Str, SerdeRmp<Worker>>,

    pub running: AtomicBool,
    pub stated_at: time::OffsetDateTime,
    pub file: data::Fs, // files

    pub processes: Box<dyn NotsBackend>,

    pub kw_secret: Secret,
    pub client: Client<hyper::client::HttpConnector>,
}

impl AppStateInner {
    pub async fn run(&self) -> Result<()> {
        use std::sync::atomic::Ordering::Relaxed;
        if self.running.load(Relaxed) {
            panic!("State is already running");
        }

        let mut tx = self.wtxn()?;

        let worker = Worker {
            app_id: "test".to_string(),
            updated_at: time::OffsetDateTime::now_utc(),
            container_id: None,
            process_id: None,
            app_version: "0.0.1".to_string(),
        };

        self.workers.put_with_flags(&mut tx, PutFlags::NO_OVERWRITE, "test", &worker)?;

        let (worker_id, worker) = self.workers.iter(&tx)?.next().unwrap()?;
        dbg!(worker_id, worker);
        tx.commit()?;

        self.running.store(true, Relaxed);
        loop {
            // if let Err(e) = self.update_apps().await {
            //     warn!("Failed to update apps: {}", e);
            // }

            tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
        }
    }

    fn rtxn(&self) -> Result<RoTxn> {
        Ok(self.db_env.read_txn()?)
    }

    fn wtxn(&self) -> Result<RwTxn> {
        Ok(self.db_env.write_txn()?)
    }

    pub(crate) async fn get_proxy_uri(&self, uri: hyper::Uri) -> hyper::Uri {
        let mut new_uri_parts = hyper::http::uri::Parts::default();
        new_uri_parts.scheme = Some("http".parse().unwrap());
        new_uri_parts.authority = Some("localhost:3333".parse().unwrap());
        new_uri_parts.path_and_query = uri.path_and_query().cloned();
        hyper::Uri::from_parts(new_uri_parts).unwrap()
    }

    async fn get_apps(&self) -> Result<HashMap<String, App>> {
        let mut wtxn = self.wtxn()?;
        let apps = self
            .apps
            .iter(&wtxn)?
            .filter_map(|res| res.ok())
            .map(|a| (a.0.to_owned(), a.1))
            .collect();
        Ok(apps)
    }
}
