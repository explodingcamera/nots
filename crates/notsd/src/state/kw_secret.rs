use aes_kw::KekAes256;
use color_eyre::eyre::{Context, Result};
use nots_client::EncryptedBytes;
use zeroize::{Zeroize, ZeroizeOnDrop, Zeroizing};

#[derive(Clone, Zeroize, ZeroizeOnDrop)]
pub struct KWSecret(String);

impl KWSecret {
    pub fn new(kw_secret: String) -> Self {
        if kw_secret.len() < 32 {
            panic!("kw_secret must be at least 32 characters long");
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
