use std::{path::PathBuf, str::FromStr};

use color_eyre::eyre::{eyre, Result};

pub enum TransportSettings {
    #[cfg(feature = "ssh")]
    Ssh(SshSettings),

    Http(HttpSettings),

    #[cfg(feature = "tls")]
    Https(HttpSettings),
}

pub struct HttpSettings {
    pub host: String,
    pub port: u16,
}

#[cfg(feature = "ssh")]
pub struct SshSettings {
    pub username: String,
    pub host: String,
    pub port: u16,

    pub local_port: u16,
}

pub struct UnixSettings {
    pub path: PathBuf,
}

enum ClientTransport {
    #[cfg(feature = "ssh")]
    Ssh {
        client: hyper::Client<HttpConnector>,
        settings: SshSettings,
    },
    #[cfg(feature = "tls")]
    Https {
        client: reqwest::Client,
        settings: HttpSettings,
    },
    Http {
        client: reqwest::Client,
        settings: HttpSettings,
    },
}

pub struct Client {
    transport: ClientTransport,
}

impl Client {
    pub fn printable_client_uri(&self) -> String {
        match &self.transport {
            #[cfg(feature = "ssh")]
            ClientTransport::Ssh { settings, .. } => {
                format!("ssh://{}@{}", settings.username, settings)
            }
            #[cfg(feature = "tls")]
            ClientTransport::Https { settings, .. } => {
                format!("https://{}:{}", settings.host, settings.port)
            }
            ClientTransport::Http { settings, .. } => {
                format!("http://{}:{}", settings.host, settings.port)
            }
        }
    }

    fn real_client_url(&self) -> String {
        match &self.transport {
            #[cfg(feature = "ssh")]
            ClientTransport::Ssh { settings, .. } => {
                format!("{}:{}", "localhost:", settings.port)
            }
            #[cfg(feature = "tls")]
            ClientTransport::Https { settings, .. } => {
                format!("{}:{}", settings.host, settings.port)
            }
            ClientTransport::Http { settings, .. } => {
                format!("{}:{}", settings.host, settings.port)
            }
        }
    }

    pub fn try_new(settings: TransportSettings) -> Result<Self> {
        let transport = match settings {
            #[cfg(feature = "tls")]
            TransportSettings::Https(settings) => ClientTransport::Https {
                client: crate::utils::create_https_client(true)?,
                settings,
            },

            TransportSettings::Http(settings) => ClientTransport::Http {
                client: crate::utils::create_http_only_client()?,
                settings,
            },
        };

        Ok(Self { transport })
    }

    fn get_client(&self) -> &reqwest::Client {
        match &self.transport {
            #[cfg(feature = "ssh")]
            ClientTransport::Ssh { client, .. } => client,
            #[cfg(feature = "tls")]
            ClientTransport::Https { client, .. } => client,
            ClientTransport::Http { client, .. } => client,
        }
    }

    pub fn req(&self, method: &str, path: &str) -> Result<reqwest::RequestBuilder> {
        let client = self.get_client();
        if !path.starts_with('/') {
            return Err(eyre!("Path must start with /"));
        }

        let method = reqwest::Method::from_str(method)?;
        let uri = format!("{}{}", self.real_client_url(), path);
        let req = client
            .request(method, uri)
            .header("x-nots-client-version", env!("CARGO_PKG_VERSION"));
        Ok(req)
    }

    pub fn http(host: &str, port: u16) -> Result<Self> {
        Self::try_new(TransportSettings::Http(HttpSettings {
            host: host.to_string(),
            port,
        }))
    }

    #[cfg(feature = "tls")]
    pub fn https(host: &str, port: u16) -> Result<Self> {
        Self::try_new(TransportSettings::Https(HttpSettings {
            host: host.to_string(),
            port,
        }))
    }
}
