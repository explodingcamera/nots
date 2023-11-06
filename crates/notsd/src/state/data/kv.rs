use color_eyre::eyre::Result;
use opendal::Operator as Op;

#[derive(Clone)]
pub struct Kv(pub Op);

impl Kv {
    pub async fn stat(&self, path: &str) -> Result<Option<opendal::Metadata>> {
        match self.0.stat(path).await {
            Ok(meta) => Ok(Some(meta)),
            Err(e) => {
                if e.kind() == opendal::ErrorKind::NotFound {
                    Ok(None)
                } else {
                    Err(e.into())
                }
            }
        }
    }

    pub async fn write<T>(&self, path: &str, value: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        let value = rmp_serde::to_vec(value)?;
        self.0.write(path, value).await?;
        Ok(())
    }

    pub async fn read<'a, T>(&self, path: &'a str) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let value = self.0.read(path).await?;
        let value: T = rmp_serde::from_slice(&value)?;
        Ok(value)
    }
}
