use std::collections::HashMap;

use bollard::{
    container::{CreateContainerOptions, ListContainersOptions, StartContainerOptions},
    image::CreateImageOptions,
    service::{ContainerInspectResponse, HostConfig},
    volume::CreateVolumeOptions,
};
use color_eyre::eyre::{bail, Result};
use futures::StreamExt;
use spinoff::{spinners, Spinner};

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

    async fn create_notsd_container(&self, version: &str, port: u16, secret: &str) -> Result<()> {
        let mut voulmes_spinner =
            Spinner::new(spinners::Dots, "Creating volumes...", spinoff::Color::Green);

        let worker_api_volume = self
            .client
            .create_volume(CreateVolumeOptions {
                driver: "local".to_string(),
                name: "notsd-worker-api".to_string(),
                labels: HashMap::from([("notsd".to_string(), "worker-api".to_string())]),
                ..Default::default()
            })
            .await
            .map_err(|err| {
                voulmes_spinner.fail("Failed to create worker-api volume");
                err
            })?;

        let db_volume = self
            .client
            .create_volume(CreateVolumeOptions {
                driver: "local".to_string(),
                name: "notsd-db".to_string(),
                labels: HashMap::from([("notsd".to_string(), "db".to_string())]),
                ..Default::default()
            })
            .await
            .map_err(|err| {
                voulmes_spinner.fail("Failed to create db volume");
                err
            })?;

        let code_volume = self
            .client
            .create_volume(CreateVolumeOptions {
                driver: "local".to_string(),
                name: "notsd-code".to_string(),
                labels: HashMap::from([("notsd".to_string(), "code".to_string())]),
                ..Default::default()
            })
            .await
            .map_err(|err| {
                voulmes_spinner.fail("Failed to create code volume");
                err
            })?;

        voulmes_spinner.stop();

        let repo = "ghcr.io/explodingcamera/notsd".to_string();
        let tag = version.to_string();
        let image = format!("{}:{}", repo, tag);

        let mut image_spinner = Spinner::new(
            spinners::Dots,
            "Pulling latest docker image...",
            spinoff::Color::Green,
        );
        let mut pull_image = self.client.create_image(
            Some(CreateImageOptions {
                from_image: image.clone(),
                ..Default::default()
            }),
            None,
            None,
        );

        // wait for image to pull
        while let Some(event) = pull_image.next().await {
            match event {
                Ok(event) => {
                    // let current = event
                    //     .progress_detail
                    //     .clone()
                    //     .unwrap_or_default()
                    //     .current
                    //     .unwrap_or_default();

                    // let total = event
                    //     .progress_detail
                    //     .unwrap_or_default()
                    //     .total
                    //     .unwrap_or_default();

                    // image_spinner.update_text(format!(
                    //     "Pulling latest docker image... {} / {}",
                    //     current, total
                    // ));
                }
                Err(err) => {
                    image_spinner.fail("Failed to pull image");
                    println!("You might need to run `docker logout ghcr.io` and try again");
                    bail!("Failed to pull image: {:?}", err);
                }
            }
        }
        image_spinner.stop();

        let port_bindings = HashMap::from([(
            "8080/tcp".to_string(),
            Some(vec![bollard::service::PortBinding {
                host_ip: None,
                host_port: Some(port.to_string()),
            }]),
        )]);

        let mut container_spinner = Spinner::new(
            spinners::Dots,
            "Starting container...",
            spinoff::Color::Green,
        );

        let container = self
            .client
            .create_container(
                Some(CreateContainerOptions {
                    name: "notsd".to_string(),
                    ..Default::default()
                }),
                bollard::container::Config {
                    image: Some(image),
                    env: Some(vec![
                        "NOTS_WORKER_API=/worker-api".to_string(),
                        "NOTS_DB=/db".to_string(),
                        "NOTS_CODE=/code".to_string(),
                        format!("NOTS_SECRET={}", secret),
                    ]),
                    host_config: Some(HostConfig {
                        port_bindings: Some(port_bindings),
                        binds: Some(vec![
                            format!("{}:/worker-api", worker_api_volume.name),
                            format!("{}:/db", db_volume.name),
                            format!("{}:/code", code_volume.name),
                        ]),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
            )
            .await
            .map_err(|err| {
                container_spinner.stop();
                err
            })?;

        // wait for container to start
        self.client
            .start_container(&container.id, None::<StartContainerOptions<String>>)
            .await
            .map_err(|err| {
                container_spinner.stop();
                err
            })?;

        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        container_spinner.stop();
        println!("");

        Ok(())
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

    async fn create(&self, version: &str, port: u16, secret: &str) -> Result<()> {
        let exists = self.find_notsd_container().await?.is_some();
        if exists {
            bail!("Notsd container already exists");
        }

        self.create_notsd_container(version, port, secret).await?;
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
