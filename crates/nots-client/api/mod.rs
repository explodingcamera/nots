use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateAppRequest {}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerStatus {
    pub version: String,
    pub uptime_secs: u64,
}

// #[derive(Serialize, Deserialize)]
// pub enum AppLocation {
//     Git {
//         id: String,
//         path: Option<String>,
//         branch: Option<String>,
//         commit: Option<String>,
//         current_commit: Option<String>,
//         current_commit_date: Option<String>,
//     },
//     Url {
//         url: String,
//     },
//     Bundle {
//         url: String,
//     },
//     Container {
//         image: String,
//         port: u16,
//     },
//     Text {
//         text: String,
//     },
// }

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
