use color_eyre::eyre::Result;
use opendal::Operator;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Runtime {
    Bun,
    Container,
}

#[derive(Serialize, Deserialize)]
pub struct Code {
    pub hostname: Option<String>, // or respond to all
    pub path: String,

    pub location: CodeLocation,
    pub entrypoint: Option<String>,
    pub runtime: Runtime,
    pub update_interval: Option<u64>,
}

#[derive(Serialize, Deserialize)]
pub enum CodeLocation {
    Git {
        id: String,
        path: String,
        branch: String,
    },
    Url {
        url: String,
    },
    Container {
        image: String,
        port: u16,
    },
}

#[derive(Serialize, Deserialize)]
pub enum Repo {
    PublicHttps { url: String },
    DeployKey { url: String, id: String },
    MachineUser { url: String, id: String },
}

#[derive(Serialize, Deserialize)]
pub enum SSHKeyType {
    Ed25519,
}

#[derive(Serialize, Deserialize)]
pub struct DeployKey {
    pub id: String,
    pub kind: SSHKeyType,
    pub key: String,
}

pub struct Data {
    pub op: Operator,
}

impl Data {
    pub fn new(op: Operator) -> Self {
        Self { op }
    }

    pub fn new_with_persy(path: &str) -> Result<Self> {
        let op = persy_operator(path)?;
        Ok(Self::new(op))
    }

    pub async fn set_deploy_key(&self, key: &DeployKey) -> Result<()> {
        self.arr_append("deploy_keys", &key.id).await?;
        self.write(&format!("deploy_key::{}", key.id), key).await?;
        Ok(())
    }

    pub async fn get_deploy_key(&self, id: &str) -> Result<DeployKey> {
        let key = self.read(&format!("deploy_key::{}", id)).await?;
        Ok(key)
    }

    pub async fn set_repo(&self, repo: &Repo, id: &str) -> Result<()> {
        self.arr_append("repos", id).await?;
        self.write(&format!("repo::{}", id), repo).await?;
        Ok(())
    }

    async fn arr_append(&self, key: &str, value: &str) -> Result<()> {
        let mut current: Vec<String> = self.read(key).await?;
        if !current.contains(&value.to_string()) {
            current.push(value.to_string());
            self.write(key, &current).await?;
        }
        Ok(())
    }
    async fn arr_remove(&self, key: &str, value: &str) -> Result<()> {
        let mut current: Vec<String> = self.read(key).await?;
        current.retain(|x| x != value);
        self.write(key, &current).await?;
        Ok(())
    }

    async fn write<T>(&self, key: &str, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        let value = serde_json::to_string(value)?;
        self.op.write(key, value).await?;
        Ok(())
    }

    async fn read<'a, T>(&self, key: &'a str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let value = self.op.read(key).await?;
        let value: T = serde_json::from_slice(&value)?;
        Ok(value)
    }
}

fn persy_operator(path: &str) -> Result<opendal::Operator> {
    let mut builder = opendal::services::Persy::default();
    builder.datafile(path);
    builder.segment("data");
    builder.index("index");

    let op: Operator = Operator::new(builder)?.finish();
    Ok(op)
}
