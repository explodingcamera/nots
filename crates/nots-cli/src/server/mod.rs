#[cfg(feature = "docker")]
pub mod docker;
use color_eyre::eyre::Result;
#[cfg(feature = "docker")]
pub use docker::DockerBackend;

#[cfg(feature = "systemd")]
mod systemd;

pub struct NotsdProcess {
    pub status: String,
    pub runtime: String,
    pub id: String,
}

#[async_trait::async_trait]
pub trait ServerBackend {
    async fn is_supported(&self) -> bool;

    async fn get(&self) -> Result<Option<NotsdProcess>>;
    async fn create(&self, version: &str, port: u16, secret: &str) -> Result<()>;
    async fn stop(&self) -> Result<()>;
    async fn start(&self) -> Result<()>;
    async fn remove(&self) -> Result<()>;
    async fn update(&self) -> Result<()>;
    async fn restart(&self) -> Result<()>;
    async fn status(&self) -> Result<()>;
}
