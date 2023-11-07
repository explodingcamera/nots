use bollard::{container::ListContainersOptions, service::ContainerInspectResponse};
use color_eyre::eyre::{bail, Result};

use super::{NotsdProcess, ServerBackend};

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

    pub async fn find_notsd_container(&self) -> Result<Option<ContainerInspectResponse>> {
        let containers = self
            .client
            .list_containers(Some(ListContainersOptions::<String> {
                all: true,
                ..Default::default()
            }))
            .await?;

        let notsd_containers = containers
            .into_iter()
            .filter(|container| {
                container
                    .names
                    .as_ref()
                    .unwrap()
                    .iter()
                    .any(|name| name == "/notsd")
            })
            .map(|container| container)
            .collect::<Vec<_>>();

        if notsd_containers.is_empty() {
            return Ok(None);
        }

        if notsd_containers.len() > 1 {
            println!("Found more than one notsd container");
            for container in notsd_containers {
                println!("  {:?}: {:?}", container.image, container.id);
            }
            bail!("Please remove the extra containers and try again");
        }

        let notsd_containers = notsd_containers.into_iter().next().expect("unreachable");
        let Some(id) = notsd_containers.id else {
            bail!("Could not find container id");
        };
        let inspect = self.client.inspect_container(&id, None).await?;
        Ok(Some(inspect))
    }
}

#[async_trait::async_trait]
impl ServerBackend for DockerBackend {
    async fn is_supported(&self) -> bool {
        self.client.ping().await.is_ok()
    }

    async fn get(&self) -> Result<Option<NotsdProcess>> {
        let Some(container) = self.find_notsd_container().await? else {
            return Ok(None);
        };

        let status = container
            .state
            .expect("container state is missing")
            .status
            .expect("container status is missing")
            .to_string();

        Ok(Some(NotsdProcess {
            id: container.id.expect("container id is missing"),
            status,
            runtime: "docker".to_string(),
        }))
    }

    async fn create(&self) -> Result<()> {
        // todo!()
        Ok(())
    }

    async fn stop(&self) -> Result<()> {
        todo!()
    }

    async fn start(&self) -> Result<()> {
        todo!()
    }

    async fn remove(&self) -> Result<()> {
        todo!()
    }

    async fn update(&self) -> Result<()> {
        todo!()
    }

    async fn restart(&self) -> Result<()> {
        todo!()
    }

    async fn status(&self) -> Result<()> {
        todo!()
    }
}
