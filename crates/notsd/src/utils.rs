use aes_kw::KekAes256;
use color_eyre::eyre::{Context, Result};

use axum::extract::connect_info;

use nots_client::EncryptedBytes;
use std::sync::Arc;
use tokio::{
    net::{unix::UCred, UnixStream},
    task::JoinSet,
};
use tracing::error;
use zeroize::{Zeroize, ZeroizeOnDrop, Zeroizing};

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
        let data = key.wrap_with_padding_vec(&data).wrap_err("Could not encrypt")?;
        Ok(EncryptedBytes(data))
    }

    pub fn decrypt(&self, data: &EncryptedBytes, id: &str) -> Result<Zeroizing<Vec<u8>>> {
        let key = KekAes256::from(self.key(id.as_bytes()));
        let res = key.unwrap_with_padding_vec(&data.0).wrap_err("Could not decrypt")?;

        Ok(Zeroizing::new(res))
    }
}

#[async_trait::async_trait]
pub trait AwaitAll {
    async fn await_all(&mut self, msg: &str) -> Result<()>;
}

#[async_trait::async_trait]
impl AwaitAll for JoinSet<Result<()>> {
    async fn await_all(&mut self, msg: &str) -> Result<()> {
        while let Some(res) = self.join_next().await {
            match res {
                Err(e) => error!("{msg}: Paniced: {}", e),
                Ok(Err(e)) => error!("{msg}: Error: {}", e),
                Ok(Ok(())) => {}
            }
        }
        Ok(())
    }
}
