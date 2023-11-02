use std::cell::OnceCell;

use color_eyre::eyre::eyre;
use color_eyre::eyre::Result;

#[cfg(feature = "ssh")]
pub mod ssh;

pub mod tcp;

#[cfg(unix)]
pub mod unix;

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
            TransportSettings::Ssh(settings) => {
                unimplemented!()
                // let transport = ssh::SshTransport::connect(settings)?;
                // Box::new(transport) as Box<dyn Transport>
            }
            TransportSettings::Tcp(settings) => {
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
