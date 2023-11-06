use std::cell::OnceCell;

use color_eyre::eyre::eyre;
use color_eyre::eyre::Result;

#[cfg(feature = "ssh")]
pub mod ssh;

pub mod tcp;

#[cfg(unix)]
pub mod unix;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct App {
    pub hostname: Option<String>, // or respond to all
    pub path: String,
    pub entrypoint: Option<String>,
    // pub location: AppLocation,
    pub run_on: Option<String>, // pin to a specific server

    pub runtime: String,
    pub runtime_version: Option<String>,
    pub runtime_settings: Option<HashMap<String, String>>,

    pub update_interval: Option<u64>, // none to disable updates
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

pub struct Client {
    settings: TransportSettings,
    transport: OnceCell<Box<dyn Transport>>,
}

pub enum TransportSettings {
    #[cfg(feature = "ssh")]
    Ssh(ssh::SshSettings),
    Tcp(tcp::TcpSettings),
    #[cfg(unix)]
    Unix(unix::UnixSettings),
}

impl Client {
    pub fn new(settings: TransportSettings) -> Self {
        Self {
            transport: OnceCell::new(),
            settings,
        }
    }

    pub async fn connect(&self) -> Result<()> {
        let transport = match &self.settings {
            #[cfg(feature = "ssh")]
            TransportSettings::Ssh(_settings) => {
                unimplemented!()
                // let transport = ssh::SshTransport::connect(settings)?;
                // Box::new(transport) as Box<dyn Transport>
            }
            TransportSettings::Tcp(_settings) => {
                unimplemented!()
                // let transport = tcp::TcpTransport::connect(settings)?;
                // Box::new(transport) as Box<dyn Transport>
            }
            #[cfg(unix)]
            TransportSettings::Unix(settings) => {
                let transport = unix::UnixTransport::connect(settings).await?;
                Box::new(transport) as Box<dyn Transport>
            }
        };

        self.transport
            .set(transport)
            .map_err(|_| eyre!("Transport already set, connect can only be called once"))?;

        Ok(())
    }
}

pub trait Transport {}
