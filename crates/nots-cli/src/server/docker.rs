use bollard::{container::ListContainersOptions, service::ContainerSummary};
use color_eyre::eyre::Result;

use super::ServerBackend;

pub struct DockerBackend {
    client: bollard::Docker,
}

impl Default for DockerBackend {
    fn default() -> Self {
        let client = bollard::Docker::connect_with_local_defaults().unwrap();
        Self { client }
    }
}

impl DockerBackend {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn get_container_by_name(&self, name: &str) -> Result<Option<ContainerSummary>> {
        let opts = ListContainersOptions::<String> {
            all: true,
            limit: None,
            size: false,
            ..Default::default()
        };

        let containers = self.client.list_containers(Some(opts)).await?;
        for container in containers {
            if let Some(names) = container.names.clone() {
                for n in names {
                    if n == name {
                        return Ok(Some(container));
                    }
                }
            }
        }
        Ok(None)
    }
}

impl ServerBackend for DockerBackend {
    fn create(&self) -> Result<(), String> {
        todo!()
    }

    fn stop(&self) -> Result<(), String> {
        todo!()
    }

    fn start(&self) -> Result<(), String> {
        todo!()
    }

    fn remove(&self) -> Result<(), String> {
        todo!()
    }

    fn update(&self) -> Result<(), String> {
        todo!()
    }

    fn restart(&self) -> Result<(), String> {
        todo!()
    }

    fn status(&self) -> Result<(), String> {
        todo!()
    }
}
