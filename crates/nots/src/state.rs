use crate::data;
use aes_kw::KekAes256;
use color_eyre::eyre::{Context, Result};
use hyper::Client;
use zeroize::{Zeroize, ZeroizeOnDrop};

#[derive(Clone)]
pub struct AppState {
    pub data: data::Data,
    pub kw_secret: KWSecret,
    pub client: Client<hyper::client::HttpConnector>,
}

#[derive(Clone, Zeroize, ZeroizeOnDrop)]
pub struct KWSecret(String);

impl KWSecret {
    fn key(&self, salt: &[u8]) -> [u8; 32] {
        let mut output_key_material = [0u8; 32];
        argon2::Argon2::default()
            .hash_password_into(self.0.as_bytes(), &salt, &mut output_key_material)
            .expect("Could not hash kw_secret");
        output_key_material
    }

    pub fn encrypt(&self, data: &str, id: &str) -> Result<Vec<u8>> {
        let key = KekAes256::from(self.key(id.as_bytes()));

        key.wrap_with_padding_vec(data.as_bytes())
            .wrap_err("Could not encrypt")
    }

    pub fn decrypt(&self, data: &[u8], id: &str) -> Result<String> {
        let key = KekAes256::from(self.key(id.as_bytes()));
        let res = key
            .unwrap_with_padding_vec(data)
            .wrap_err("Could not decrypt")?;

        String::from_utf8(res).wrap_err("Could not decrypt")
    }
}

impl AppState {
    pub fn new(data: data::Data, kw_secret: String) -> Self {
        if kw_secret.len() < 32 {
            panic!("kw_secret must be at least 32 characters long");
        }

        Self {
            data,
            kw_secret: KWSecret(kw_secret),
            client: Client::default(),
        }
    }
}
