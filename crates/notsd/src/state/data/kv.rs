use color_eyre::eyre::Result;
use opendal::Operator as Op;

#[derive(Clone)]
pub struct Operator(pub Op);

impl Operator {
    pub async fn write<T>(&self, key: &str, value: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        let value = rmp_serde::to_vec(value)?;
        self.0.write(key, value).await?;
        Ok(())
    }

    pub async fn read<'a, T>(&self, key: &'a str) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let value = self.0.read(key).await?;
        let value: T = rmp_serde::from_slice(&value)?;
        Ok(value)
    }
}
