use std::cell::OnceCell;

use color_eyre::eyre::Result;
use hyper::{Body, Request, Response};

#[cfg(feature = "ssh")]
pub mod ssh;

pub mod tcp;

#[cfg(unix)]
pub mod unix;

pub enum TransportSettings {
    #[cfg(feature = "ssh")]
    Ssh(ssh::SshSettings),
    Tcp(tcp::TcpSettings),
    #[cfg(unix)]
    Unix(unix::UnixSettings),
}

pub struct Client {
    settings: TransportSettings,
    transport: OnceCell<Box<dyn Transport>>,
}

impl Client {
    pub fn new(settings: TransportSettings) -> Self {
        Self {
            transport: OnceCell::new(),
            settings,
        }
    }

    pub fn request(&self, request: Request<Body>) -> Result<Response<Body>> {
        todo!()
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

        self.transport.set(transport).map_err(|_| {
            color_eyre::eyre::eyre!("Transport already set, connect can only be called once")
        })?;

        Ok(())
    }
}

pub trait Transport {}
