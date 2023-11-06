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

#[derive(Serialize, Deserialize, Clone)]
pub struct WorkerState {
    pub status: WorkerStatus,
    pub restart_count: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone)]
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
    pub hostname: Option<String>, // or respond to all
    pub path: String,

    pub worker_settings: WorkerSettings,
    pub worker_runtime: WorkerRuntimeOptions,
}
