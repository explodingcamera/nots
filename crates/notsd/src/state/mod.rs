mod db;

pub use db::fs_operator;
use nots_client::models::{App, WorkerState};
use okv::{rocksdb::RocksDbOptimistic, types::serde::SerdeRmp, Database};
use tokio::task::JoinSet;

use crate::{
    backend::NotsBackend,
    utils::{AwaitAll, Secret},
};
use color_eyre::eyre::Result;
use hyper::Client;
use opendal::Operator;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{atomic::AtomicBool, Arc},
};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Worker {
    pub app_id: String,
    pub state: WorkerState,
    pub updated_at: time::OffsetDateTime,

    pub container_id: Option<String>,
    pub process_id: Option<u32>,

    pub app_version: String,
}

pub type AppState = Arc<AppStateInner>;

pub async fn try_new(
    db_env: okv::Env<RocksDbOptimistic>,
    file: Operator,
    kw_secret: &str,
    processes: Box<dyn NotsBackend>,
) -> Result<AppState> {
    if kw_secret.len() < 16 {
        panic!("kw_secret must be at least 16 characters long");
    }

    let file = db::Fs(file);

    let apps = db_env.open("apps")?;
    let node_id = "1";
    let workers = db_env.open(&format!("workers-{}", node_id))?;

    Ok(AppStateInner {
        db_env: db_env.clone(),
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
    pub db_env: okv::Env<RocksDbOptimistic>,
    pub apps: Database<String, SerdeRmp<App>, RocksDbOptimistic>,
    pub workers: Database<String, SerdeRmp<Worker>, RocksDbOptimistic>,

    pub running: AtomicBool,
    pub stated_at: time::OffsetDateTime,
    pub file: db::Fs, // files

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

        self.running.store(true, Relaxed);
        let apps = self.get_apps()?;
        let workers = self.get_workers()?;
        let mut joinset = JoinSet::new();

        loop {
            // check for invalid workers (not running, not needed)
            for (id, w) in workers.iter() {
                let Some(app) = apps.get(&w.app_id) else {
                    unimplemented!("Clean up invalid workers");
                };

                unimplemented!("update worker state");

                if app.needs_restart_since.unwrap_or(time::OffsetDateTime::UNIX_EPOCH) > w.updated_at {
                    unimplemented!("Restart worker");
                }

                joinset.spawn(async move { Result::<()>::Ok(()) });
            }
            joinset.await_all("workers").await?;

            // check for new apps that need workers
            for (aid, app) in apps.iter() {}

            tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
        }
    }

    pub(crate) fn get_proxy_uri(&self, uri: hyper::Uri) -> hyper::Uri {
        let mut new_uri_parts = hyper::http::uri::Parts::default();
        new_uri_parts.scheme = Some("http".parse().unwrap());
        new_uri_parts.authority = Some("localhost:3333".parse().unwrap());
        new_uri_parts.path_and_query = uri.path_and_query().cloned();
        hyper::Uri::from_parts(new_uri_parts).unwrap()
    }

    fn delete_worker(&self, id: &str) -> Result<()> {
        Ok(self.workers.delete(id)?)
    }

    fn set_worker(&self, id: &str, worker: Worker) -> Result<()> {
        Ok(self.workers.set(id, &worker)?)
    }

    fn get_workers(&self) -> Result<Vec<(String, Worker)>> {
        unimplemented!();

        let workers = self
            .workers
            .iter()?
            .filter_map(|res| res.ok())
            .map(|a| (a.0.to_owned(), a.1))
            .collect();

        Ok(workers)
    }

    fn create_app(&self, app: App) -> Result<Option<String>> {
        let id = cuid2::cuid();
        Ok(self.apps.set(&id, &app).map(|_| Some(id))?)
    }

    fn get_app(&self, app_id: &str) -> Result<Option<App>> {
        let app = self.apps.get(app_id)?;
        Ok(app)
    }

    fn get_apps(&self) -> Result<HashMap<String, App>> {
        let apps = self
            .apps
            .iter()?
            .filter_map(|res| res.ok())
            .map(|a| (a.0.to_owned(), a.1))
            .collect();

        Ok(apps)
    }
}
