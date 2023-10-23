use crate::data;
use hyper::Client;
use zeroize::{Zeroize, ZeroizeOnDrop};

#[derive(Clone)]
pub struct AppState {
    pub data: data::Data,
    kw_secret: KWSecret,
    pub client: Client<hyper::client::HttpConnector>,
}

#[derive(Clone, Zeroize, ZeroizeOnDrop)]
struct KWSecret(String);

impl KWSecret {
    pub fn key(&self, salt: &[u8]) -> [u8; 32] {
        let mut output_key_material = [0u8; 32];
        argon2::Argon2::default()
            .hash_password_into(self.0.as_bytes(), &salt, &mut output_key_material)
            .expect("Could not hash kw_secret");
        output_key_material
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
