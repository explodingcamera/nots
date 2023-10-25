use axum::{extract::connect_info, http::HeaderValue, BoxError};
use hyper::{server::accept::Accept, HeaderMap};
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tokio::net::{unix::UCred, UnixListener, UnixStream};

pub fn add_x_forwarded_for(headers: &mut HeaderMap<HeaderValue>, addr: SocketAddr) {
    let client_ip = addr.ip().to_string();
    if let Some(existing_header) = headers.get("X-Forwarded-For") {
        // Append the client IP address if the header already exists
        let updated_header = format!("{}, {}", existing_header.to_str().unwrap_or(""), client_ip);
        headers.insert("X-Forwarded-For", updated_header.parse().unwrap());
    } else {
        // Add a new header if it doesn't already exist
        headers.insert("X-Forwarded-For", client_ip.parse().unwrap());
    }
}

pub fn remove_hop_by_hop_headers(headers: &mut HeaderMap<HeaderValue>) {
    headers.remove("connection");
    headers.remove("keep-alive");
    headers.remove("proxy-authenticate");
    headers.remove("proxy-authorization");
    headers.remove("te");
    headers.remove("trailers");
    headers.remove("transfer-encoding");
    headers.remove("upgrade");
}

pub(crate) async fn create_unix_socket(path: PathBuf) -> ServerAccept {
    let _ = tokio::fs::remove_file(&path).await;

    tokio::fs::create_dir_all(
        path.parent()
            .expect(format!("Could not get parent of {}", path.display()).as_str()),
    )
    .await
    .expect(format!("Could not create directory {}", path.display()).as_str());
    let listener = tokio::net::UnixListener::bind(path.clone())
        .expect(format!("Could not bind to {}", path.display()).as_str());
    ServerAccept { uds: listener }
}

pub(crate) struct ServerAccept {
    uds: UnixListener,
}

impl Accept for ServerAccept {
    type Conn = UnixStream;
    type Error = BoxError;

    fn poll_accept(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Result<Self::Conn, Self::Error>>> {
        let (stream, _addr) = std::task::ready!(self.uds.poll_accept(cx))?;
        std::task::Poll::Ready(Some(Ok(stream)))
    }
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub(crate) struct UdsConnectInfo {
    peer_addr: Arc<tokio::net::unix::SocketAddr>,
    peer_cred: UCred,
}

impl connect_info::Connected<&UnixStream> for UdsConnectInfo {
    fn connect_info(target: &UnixStream) -> Self {
        let peer_addr = target.peer_addr().unwrap();
        let peer_cred = target.peer_cred().unwrap();

        Self {
            peer_addr: Arc::new(peer_addr),
            peer_cred,
        }
    }
}
