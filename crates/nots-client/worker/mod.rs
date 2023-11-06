use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::models::WorkerSettings;

pub const SOURCE_VERSION_HEADER: &str = "x-nots-worker-source-version";
pub const WORKER_ID_HEADER: &str = "x-nots-worker-id";
pub const WORKER_VERSION_HEADER: &str = "x-nots-worker-version";

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
