use std::path::PathBuf;

use color_eyre::eyre::Result;
use hyper::{client::HttpConnector, Body, HeaderMap, Request, Response};

pub enum TransportSettings {
    #[cfg(feature = "ssh")]
    Ssh(SshSettings),

    Http(HttpSettings),

    #[cfg(feature = "tls")]
    Https(HttpSettings),

    #[cfg(unix)]
    Unix(UnixSettings),
}

pub struct HttpSettings {
    pub host: String,
    pub port: u16,
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
        client: hyper::Client<hyper_rustls::HttpsConnector<HttpConnector>>,
        settings: HttpSettings,
    },
    Http {
        client: hyper::Client<HttpConnector>,
        settings: HttpSettings,
    },
    #[cfg(unix)]
    Unix {
        client: hyper::Client<hyperlocal::UnixConnector>,
        settings: UnixSettings,
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
            #[cfg(unix)]
            ClientTransport::Unix { settings, .. } => {
                format!("unix://{}", settings.path.display())
            }
        }
    }

    pub fn new(settings: TransportSettings) -> Self {
        let transport = match settings {
            #[cfg(feature = "tls")]
            TransportSettings::Https(settings) => {
                ClientTransport::Https { client: crate::utils::create_https_client(true), settings }
            }

            TransportSettings::Http(settings) => {
                let client = hyper::Client::builder().build(HttpConnector::new());
                ClientTransport::Http { client, settings }
            }
            #[cfg(unix)]
            TransportSettings::Unix(settings) => {
                let client = hyper::Client::builder().build(hyperlocal::UnixConnector);
                ClientTransport::Unix { client, settings }
            }
        };

        Self { transport }
    }

    #[cfg(unix)]
    pub fn unix(path: PathBuf) -> Self {
        Self::new(TransportSettings::Unix(UnixSettings { path }))
    }

    pub fn http(host: &str, port: u16) -> Self {
        Self::new(TransportSettings::Http(HttpSettings { host: host.to_string(), port }))
    }

    pub fn get_uri(&self, uri: &str) -> hyper::Uri {
        let uri_path = uri
            .parse::<hyper::Uri>()
            .expect("Invalid URI")
            .path_and_query()
            .expect("Invalid URI")
            .as_str()
            .to_string();

        match &self.transport {
            ClientTransport::Http { client, settings } => {
                let addr: hyper::Uri =
                    format!("http://{}:{}{}", settings.host, settings.port, uri_path)
                        .parse()
                        .unwrap();
                addr
            }

            #[cfg(feature = "tls")]
            ClientTransport::Https { client, settings } => {
                let addr: hyper::Uri =
                    format!("https://{}:{}{}", settings.host, settings.port, uri_path)
                        .parse()
                        .unwrap();
                addr
            }

            #[cfg(unix)]
            ClientTransport::Unix { client, settings } => {
                let addr: hyper::Uri =
                    hyperlocal::Uri::new(settings.path.clone(), &uri_path).into();
                addr
            }
        }
    }

    async fn request(&self, req: Request<Body>) -> Result<Response<Body>> {
        match &self.transport {
            #[cfg(feature = "tls")]
            ClientTransport::Https { client, settings } => {
                let res = client.request(req).await?;
                Ok(res)
            }
            ClientTransport::Http { client, settings } => {
                let res = client.request(req).await?;
                Ok(res)
            }
            #[cfg(unix)]
            ClientTransport::Unix { client, settings } => {
                let res = client.request(req).await?;
                Ok(res)
            }
        }
    }

    pub async fn req_json<TReq: serde::Serialize, TRes: serde::de::DeserializeOwned>(
        &self,
        uri: &str,
        method: &str,
        body: Option<TReq>,
        headers: Option<HeaderMap>,
    ) -> Result<(TRes, hyper::http::response::Parts)> {
        let body = match body {
            Some(body) => serde_json::to_string(&body)?,
            None => "".to_string(),
        };

        let mut headers = match headers {
            Some(headers) => headers,
            None => HeaderMap::new(),
        };

        headers.insert("content-type", "application/json".parse()?);

        let res = self.req(uri, method, Some(Body::from(body)), Some(headers)).await?;
        let (parts, body) = res.into_parts();
        let body = hyper::body::to_bytes(body).await?;
        let body = String::from_utf8(body.to_vec())?;
        let body = serde_json::from_str(&body)?;
        Ok((body, parts))
    }

    pub async fn req(
        &self,
        uri: &str,
        method: &str,
        body: Option<Body>,
        headers: Option<HeaderMap>,
    ) -> Result<Response<Body>> {
        let uri = self.get_uri(uri);

        let mut req = hyper::Request::builder().method(method);

        if let Some(headers) = headers {
            *req.headers_mut().unwrap() = headers;
        }

        let mut req = req
            .header("x-nots-client-version", env!("CARGO_PKG_VERSION"))
            .uri(uri)
            .body(body.unwrap_or(Body::empty()))?;

        let res = self.request(req).await?;
        Ok(res)
    }
}
