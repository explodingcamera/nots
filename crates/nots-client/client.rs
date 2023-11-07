use std::path::PathBuf;

use color_eyre::eyre::Result;
use hyper::{client::HttpConnector, Body, HeaderMap, Request, Response};
use hyperlocal::UnixConnector;

pub enum TransportSettings {
    #[cfg(feature = "ssh")]
    Ssh(SshSettings),
    Http(HttpSettings),
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
    Http {
        client: hyper::Client<HttpConnector>,
        settings: HttpSettings,
    },
    #[cfg(unix)]
    Unix {
        client: hyper::Client<UnixConnector>,
        settings: UnixSettings,
    },
}

pub struct Client {
    transport: ClientTransport,
}

impl Client {
    pub fn new(settings: TransportSettings) -> Self {
        let transport = match settings {
            TransportSettings::Http(settings) => {
                let client = hyper::Client::builder().build(HttpConnector::new());
                ClientTransport::Http { client, settings }
            }
            #[cfg(unix)]
            TransportSettings::Unix(settings) => {
                let client = hyper::Client::builder().build(UnixConnector);
                ClientTransport::Unix { client, settings }
            }
        };

        Self { transport }
    }

    pub fn unix(path: PathBuf) -> Self {
        Self::new(TransportSettings::Unix(UnixSettings { path }))
    }

    pub fn http(host: String, port: u16) -> Self {
        Self::new(TransportSettings::Http(HttpSettings { host, port }))
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
            ClientTransport::Unix { client, settings } => {
                let addr: hyper::Uri =
                    hyperlocal::Uri::new(settings.path.clone(), &uri_path).into();
                addr
            }
        }
    }

    async fn request(&self, req: Request<Body>) -> Result<Response<Body>> {
        match &self.transport {
            ClientTransport::Http { client, settings } => {
                let res = client.request(req).await?;
                Ok(res)
            }
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
    ) -> Result<TRes> {
        let body = match body {
            Some(body) => serde_json::to_string(&body)?,
            None => "".to_string(),
        };

        let mut headers = match headers {
            Some(headers) => headers,
            None => HeaderMap::new(),
        };

        headers.insert("content-type", "application/json".parse()?);

        let res = self
            .req(uri, method, Some(Body::from(body)), Some(headers))
            .await?;
        let body = hyper::body::to_bytes(res.into_body()).await?;
        let body = String::from_utf8(body.to_vec())?;
        let res = serde_json::from_str(&body)?;
        Ok(res)
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