mod db;

pub use db::fs_operator;
use nots_client::models::{App, WorkerState};
use tokio::task::JoinSet;

use crate::{
    backend::NotsBackend,
    state::db::heed::HeedExt,
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

use heed::{
    types::{SerdeRmp, Str},
    Database, DatabaseOpenOptions, RoTxn, RwTxn,
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
    db_env: heed::Env,
    file: Operator,
    kw_secret: &str,
    processes: Box<dyn NotsBackend>,
) -> Result<AppState> {
    if kw_secret.len() < 16 {
        panic!("kw_secret must be at least 16 characters long");
    }

    let file = db::Fs(file);

    let mut wtxn = db_env.write_txn()?;

    let apps = DatabaseOpenOptions::new(&db_env)
        .types::<Str, SerdeRmp<App>>()
        .name("apps")
        .create(&mut wtxn)?;

    let node_id = "1";
    let workers = DatabaseOpenOptions::new(&db_env)
        .types::<Str, SerdeRmp<Worker>>()
        .name(format!("workers-{}", node_id))
        .create(&mut wtxn)?;

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

                if app.needs_restart_since.unwrap_or(time::OffsetDateTime::UNIX_EPOCH)
                    > w.updated_at
                {
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

    fn rtxn(&self) -> Result<RoTxn> {
        Ok(self.db_env.read_txn()?)
    }

    fn wtxn(&self) -> Result<RwTxn> {
        Ok(self.db_env.write_txn()?)
    }

    pub(crate) fn get_proxy_uri(&self, uri: hyper::Uri) -> hyper::Uri {
        let mut new_uri_parts = hyper::http::uri::Parts::default();
        new_uri_parts.scheme = Some("http".parse().unwrap());
        new_uri_parts.authority = Some("localhost:3333".parse().unwrap());
        new_uri_parts.path_and_query = uri.path_and_query().cloned();
        hyper::Uri::from_parts(new_uri_parts).unwrap()
    }

    fn delete_worker(&self, id: &str) -> Result<()> {
        let mut wtxn = self.wtxn()?;
        self.workers.delete(&mut wtxn, id)?;
        wtxn.commit()?;
        Ok(())
    }

    fn set_worker(&self, id: &str, worker: Worker) -> Result<()> {
        let mut wtxn = self.wtxn()?;
        self.workers.put(&mut wtxn, id, &worker)?;
        wtxn.commit()?;
        Ok(())
    }

    fn get_workers(&self) -> Result<Vec<(String, Worker)>> {
        let mut rtxn = self.rtxn()?;
        let workers = self
            .workers
            .iter(&rtxn)?
            .filter_map(|res| res.ok())
            .map(|a| (a.0.to_owned(), a.1))
            .collect();

        rtxn.commit()?;
        Ok(workers)
    }

    fn create_app(&self, app: App) -> Result<Option<String>> {
        let id = cuid2::cuid();
        let mut wtxn = self.wtxn()?;
        let res = self.apps.put_if_absent(&mut wtxn, &id, &app)?;
        wtxn.commit()?;
        Ok(res.map(|_| id))
    }

    fn get_app(&self, app_id: &str) -> Result<Option<App>> {
        let mut rtxn = self.rtxn()?;
        let app = self.apps.get(&rtxn, app_id)?;
        rtxn.commit()?;
        Ok(app)
    }

    fn get_apps(&self) -> Result<HashMap<String, App>> {
        let mut rtxn = self.rtxn()?;
        let apps = self
            .apps
            .iter(&rtxn)?
            .filter_map(|res| res.ok())
            .map(|a| (a.0.to_owned(), a.1))
            .collect();
        rtxn.commit()?;
        Ok(apps)
    }
}
