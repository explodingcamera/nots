use std::collections::HashMap;

use color_eyre::eyre::Result;
use opendal::Operator;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Runtime {
    Container,
    Bun,
    HostBinary,
    ContainerBinary,
}

#[derive(Serialize, Deserialize)]
pub struct App {
    settings: AppSettings,
}

#[derive(Serialize, Deserialize)]
pub struct AppSettings {
    pub hostname: Option<String>, // or respond to all
    pub path: String,
    pub location: AppLocation,
    pub entrypoint: Option<String>,
    pub runtime: Runtime,
    pub update_interval: Option<u64>, // none to disable updates
}

#[derive(Serialize, Deserialize)]
pub enum AppLocation {
    Git {
        id: String,
        path: Option<String>,
        branch: Option<String>,
        commit: Option<String>,
        current_commit: Option<String>,
        current_commit_date: Option<String>,
    },
    Url {
        url: String,
    },
    Bundle {
        url: String,
    },
    Container {
        image: String,
        port: u16,
    },
    Text {
        text: String,
    },
}

#[derive(Serialize, Deserialize)]
pub enum Repo {
    PublicHttps { url: String },
    DeployKey { url: String, id: String },
    // MachineUser { url: String, id: String },
}

#[derive(Serialize, Deserialize)]
pub enum SSHKeyType {
    Ed25519,
}

#[derive(Serialize, Deserialize)]
pub struct DeployKey {
    pub kind: SSHKeyType,
    pub key: Vec<u8>,
}

#[derive(Clone)]
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

    // APPS

    pub async fn set_app(&self, app: &AppSettings, id: &str) -> Result<()> {
        self.arr_append("apps", &id).await?;
        self.write(&format!("app::{}", id), app).await.into()
    }
    pub async fn get_app(&self, id: &str) -> Result<AppSettings> {
        self.read(&format!("app::{}", id)).await.into()
    }
    pub async fn get_apps(&self) -> Result<HashMap<String, AppSettings>> {
        let mut apps = HashMap::new();
        let ids: Vec<String> = self.read("apps").await?;
        for id in ids {
            let app = self.get_app(&id).await?;
            apps.insert(id, app);
        }
        Ok(apps)
    }

    // REPOS

    pub async fn set_repo(&self, repo: &Repo, id: &str) -> Result<()> {
        self.arr_append("repos", id).await?;
        self.write(&format!("repo::{}", id), repo).await.into()
    }
    pub async fn remove_repo(&self, id: &str) -> Result<()> {
        self.arr_remove("repos", id).await?;
        self.op.delete(&format!("repo::{}", id)).await?;
        Ok(())
    }
    pub async fn get_repo(&self, id: &str) -> Result<Repo> {
        self.read(&format!("repo::{}", id)).await.into()
    }
    pub async fn get_repos(&self) -> Result<HashMap<String, Repo>> {
        let mut repos = HashMap::new();
        let ids: Vec<String> = self.read("repos").await?;
        for id in ids {
            let repo = self.get_repo(&id).await?;
            repos.insert(id, repo);
        }
        Ok(repos)
    }

    // DEPLOY KEYS

    pub async fn get_deploy_keys(&self) -> Result<Vec<DeployKey>> {
        let mut keys = vec![];
        let ids: Vec<String> = self.read("deploy_keys").await?;
        for id in ids {
            let key = self.get_deploy_key(&id).await?;
            keys.push(key);
        }
        Ok(keys)
    }
    pub async fn set_deploy_key(&self, key: &DeployKey, id: &str) -> Result<()> {
        self.arr_append("deploy_keys", &id).await?;
        self.write(&format!("deploy_key::{}", id), key).await?;
        Ok(())
    }
    pub async fn get_deploy_key(&self, id: &str) -> Result<DeployKey> {
        let key = self.read(&format!("deploy_key::{}", id)).await?;
        Ok(key)
    }
    pub async fn remove_deploy_key(&self, id: &str) -> Result<()> {
        self.arr_remove("deploy_keys", id).await?;
        self.op.delete(&format!("deploy_key::{}", id)).await?;
        Ok(())
    }

    pub async fn arr_append(&self, key: &str, value: &str) -> Result<()> {
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
        let value = rmp_serde::to_vec(value)?;
        self.op.write(key, value).await?;
        Ok(())
    }

    async fn read<'a, T>(&self, key: &'a str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let value = self.op.read(key).await?;
        let value: T = rmp_serde::from_slice(&value)?;
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
