mod utils;
pub use utils::*;

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct EncryptedBytes(pub Vec<u8>);

pub mod app {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct App {
        settings: AppSettings,
    }

    #[derive(Serialize, Deserialize)]
    pub struct AppSettings {
        pub hostname: Option<String>, // or respond to all
        pub path: String,
        pub location: AppLocation,
        pub entrypoint: Option<String>,
        pub runtime: String,
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

    #[derive(Debug, Deserialize, Serialize)]
    pub struct WorkerSettings {
        pub port: u16,
        pub command: Option<String>,
        pub main: Option<String>,
        pub env: HashMap<String, String>,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct WorkerRegisterResponse {
        pub settings: WorkerSettings,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct WorkerHeartbeatResponse {
        pub ok: bool,
    }
}
