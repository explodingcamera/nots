use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WorkerSettings {
    pub port: Option<u16>,            // port to listen on
    pub prepare: Option<String>,      // command to run before starting the worker
    pub command: Option<String>,      // command to run to start the worker
    pub main: Option<String>,         // file to pass to the command
    pub env: HashMap<String, String>, // env vars to pass to the command
}
// pub secrets: HashMap<String, String>, // secrets available to the worker, key:

#[derive(Serialize, Deserialize, Clone)]
pub struct Worker {
    pub version: String,
    pub runtime: WorkerRuntimeOptions,
    pub settings: WorkerSettings,
    pub state: WorkerState,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum WorkerRuntimeOptions {
    Docker(DockerRuntimeOptions),
    Process {},
}

#[derive(Serialize, Deserialize, Clone)]
pub enum DockerRuntimeOptions {
    Standalone { image: String, tag: String }, // container without nots-worker
    Custom { image: String, tag: String },     // container with nots-worker
    Bun { version: String, global_cache: bool },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WorkerState {
    pub status: WorkerStatus,
    pub restart_count: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum WorkerStatus {
    Created,
    Running,
    Paused,
    Restarting,
    Removing,
    Exited,
    Dead,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct App {
    pub hostnames: Vec<Match>, // hostname to match
    pub routes: Vec<Match>,    // routes to match (ignores query string, glob is case insensitive)
    pub route_priority: i16,   // higher priority routes are matched first, default 0

    pub worker_settings: WorkerSettings,
    pub worker_runtime: WorkerRuntimeOptions,

    pub updated_at: Option<time::OffsetDateTime>,
    pub needs_restart_since: Option<time::OffsetDateTime>,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Match {
    Glob(String),
    Regex(String),
}

impl Match {
    #[cfg(feature = "glob")]
    pub fn regex(self) -> Result<String, globset::Error> {
        Ok(match self {
            Match::Glob(glob) => globset::GlobBuilder::new(&glob)
                .case_insensitive(true)
                .build()?
                .regex()
                .to_string(),
            Match::Regex(regex) => regex,
        })
    }
}
