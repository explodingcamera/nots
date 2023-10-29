mod utils;
pub use utils::*;

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct EncryptedBytes(pub Vec<u8>);

pub mod app {
    use std::collections::HashMap;

    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct AppSettings {
        pub hostname: Option<String>, // or respond to all
        pub path: String,
        pub location: AppLocation,
        pub entrypoint: Option<String>,

        pub run_on: Option<String>, // pin to a specific server

        pub runtime: String,
        pub runtime_version: Option<String>,
        pub runtime_settings: Option<HashMap<String, String>>,

        pub update_interval: Option<u64>, // none to disable updates
    }

    #[derive(Serialize, Deserialize)]
    pub enum AppLocation {
        Git {
            id: String,
            path: Option<String>,
            branch: Option<String>,
            commit: Option<String>,
            current_commit: Option<String>,
            current_commit_date: Option<String>,
        },
        Url {
            url: String,
        },
        Bundle {
            url: String,
        },
        Container {
            image: String,
            port: u16,
        },
        Text {
            text: String,
        },
    }

    #[derive(Serialize, Deserialize)]
    pub enum Repo {
        PublicHttps { url: String },
        DeployKey { url: String, id: String },
        // MachineUser { url: String, id: String },
    }

    #[derive(Serialize, Deserialize, PartialEq)]
    #[non_exhaustive]
    pub enum SSHKeyType {
        Ed25519,
    }

    #[derive(Serialize, Deserialize)]
    pub struct DeployKey {
        pub kind: SSHKeyType,
        pub key: crate::EncryptedBytes,
    }
}

pub mod worker {
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
}
