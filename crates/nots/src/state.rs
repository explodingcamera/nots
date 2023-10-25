use std::sync::Arc;

use crate::data;
use aes_kw::KekAes256;
use color_eyre::eyre::{Context, Result};
use dashmap::DashMap;
use hyper::Client;
use zeroize::{Zeroize, ZeroizeOnDrop};

#[derive(Clone)]
pub struct AppState {
    pub data: data::Data,
    pub kw_secret: Arc<KWSecret>,
    pub client: Client<hyper::client::HttpConnector>,

    pub workers: Arc<DashMap<String, Worker>>,
    pub apps: Arc<DashMap<String, App>>,
}

pub struct Worker {}
pub struct App {}

#[derive(Clone, Zeroize, ZeroizeOnDrop)]
pub struct KWSecret(String);

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct EncryptedBytes(pub Vec<u8>);
impl KWSecret {
    fn key(&self, salt: &[u8]) -> [u8; 32] {
        let mut output_key_material = [0u8; 32];
        argon2::Argon2::default()
            .hash_password_into(self.0.as_bytes(), &salt, &mut output_key_material)
            .expect("Could not hash kw_secret");
        output_key_material
    }

    pub fn encrypt(&self, data: EncryptedBytes, id: &str) -> Result<Vec<u8>> {
        let key = KekAes256::from(self.key(id.as_bytes()));
        key.wrap_with_padding_vec(&data.0)
            .wrap_err("Could not encrypt")
    }

    pub fn decrypt<'a>(&self, data: &[u8], id: &str) -> Result<EncryptedBytes> {
        let key = KekAes256::from(self.key(id.as_bytes()));
        let res = key
            .unwrap_with_padding_vec(data)
            .wrap_err("Could not decrypt")?;

        Ok(EncryptedBytes(res))
    }
}

impl AppState {
    pub fn new(data: data::Data, kw_secret: String) -> Self {
        if kw_secret.len() < 32 {
            panic!("kw_secret must be at least 32 characters long");
        }

        Self {
            data,
            kw_secret: Arc::new(KWSecret(kw_secret)),
            client: Client::default(),
            apps: Arc::new(DashMap::new()),
            workers: Arc::new(DashMap::new()),
        }
    }
}
