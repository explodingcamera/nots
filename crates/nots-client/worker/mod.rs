use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub const SOURCE_VERSION_HEADER: &str = "x-nots-worker-source-version";
pub const WORKER_ID_HEADER: &str = "x-nots-worker-id";
pub const WORKER_VERSION_HEADER: &str = "x-nots-worker-version";

#[derive(Debug, Deserialize, Serialize)]
pub struct WorkerSettings {
    pub port: u16,                    // port to listen on
    pub prepare: Option<String>,      // command to run before starting the worker
    pub command: Option<String>,      // command to run to start the worker
    pub main: Option<String>,         // file to pass to the command
    pub env: HashMap<String, String>, // env vars to pass to the command
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WorkerRegisterResponse {
    pub settings: WorkerSettings,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WorkerReadyResponse {}

#[derive(Debug, Deserialize, Serialize)]
pub struct WorkerReadyRequest {}

#[derive(Debug, Deserialize, Serialize)]
pub struct WorkerSourceRequest {
    pub source_version: Option<String>, // commit or file hash
}
