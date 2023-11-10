use aes_kw::KekAes256;
use color_eyre::eyre::{bail, Context, ContextCompat, Result};

use axum::{extract::connect_info, http::HeaderValue, BoxError};
use hyper::{server::accept::Accept, HeaderMap};

use nots_client::EncryptedBytes;
use std::{os::unix::fs::chown, path::PathBuf, sync::Arc};
use tokio::net::{unix::UCred, UnixListener, UnixStream};
use tracing::warn;
use zeroize::{Zeroize, ZeroizeOnDrop, Zeroizing};

pub(crate) async fn create_unix_socket(path: PathBuf) -> Result<ServerAccept> {
    let _ = tokio::fs::remove_file(&path).await;

    tokio::fs::create_dir_all(
        path.parent()
            .unwrap_or_else(|| panic!("Could not get parent of {}", path.display())),
    )
    .await
    .unwrap_or_else(|_| panic!("Could not create directory {}", path.display()));

    let listener = tokio::net::UnixListener::bind(path.clone())
        .unwrap_or_else(|_| panic!("Could not bind to {}", path.display()));

    let uid = std::env::var("NOTS_SOCK_UID");
    let gid = std::env::var("NOTS_SOCK_GID");

    if let (Ok(uid), Ok(gid)) = (uid, gid) {
        let uid = uid
            .parse::<u32>()
            .context("Could not parse NOTS_SOCK_UID")?;
        let gid = gid
            .parse::<u32>()
            .context("Could not parse NOTS_SOCK_GID")?;

        chown(&path, Some(uid), Some(gid)).context("Could not chown socket")?;
    } else if cfg!(debug_assertions) {
        // prob. local, set to nots group and current user
        warn!("No NOTS_SOCK_UID, NOTS_SOCK_GID");
        let gid = nix::unistd::Group::from_name("nots")?
            .context("Could not get nots group")?
            .gid;
        chown(&path, None, Some(gid.into())).context("Could not chown socket")?;
    } else {
        bail!("No NOTS_SOCK_UID, NOTS_SOCK_GID. Please set these environment variables");
    }

    Ok(ServerAccept { uds: listener })
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

#[derive(Clone, Zeroize, ZeroizeOnDrop)]
pub struct Secret(String);

impl Secret {
    pub fn new(kw_secret: String) -> Self {
        if kw_secret.len() < 16 {
            panic!("kw_secret must be at least 16 characters long");
        }
        Self(kw_secret)
    }

    fn key(&self, salt: &[u8]) -> [u8; 32] {
        let mut output_key_material = [0u8; 32];
        argon2::Argon2::default()
            .hash_password_into(self.0.as_bytes(), salt, &mut output_key_material)
            .expect("Could not hash kw_secret");
        output_key_material
    }

    pub fn encrypt(&self, data: Zeroizing<Vec<u8>>, id: &str) -> Result<EncryptedBytes> {
        let key = KekAes256::from(self.key(id.as_bytes()));
        let data = key
            .wrap_with_padding_vec(&data)
            .wrap_err("Could not encrypt")?;
        Ok(EncryptedBytes(data))
    }

    pub fn decrypt(&self, data: &EncryptedBytes, id: &str) -> Result<Zeroizing<Vec<u8>>> {
        let key = KekAes256::from(self.key(id.as_bytes()));
        let res = key
            .unwrap_with_padding_vec(&data.0)
            .wrap_err("Could not decrypt")?;

        Ok(Zeroizing::new(res))
    }
}
