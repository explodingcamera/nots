use std::collections::HashMap;

use super::{CreateWorker, WorkerState, WorkerStatus};
use crate::runtime::NotsRuntime;
use axum::async_trait;
use bollard::{
    container::*,
    network::CreateNetworkOptions,
    service::{ContainerSummary, Ipam},
};
use color_eyre::eyre::{bail, Result};
use nots_client::models::{DockerRuntimeOptions, WorkerRuntimeOptions};

pub struct DockerBackendSettings {
    pub worker_prefix: String,
    pub worker_labels: HashMap<String, String>,
}

impl Default for DockerBackendSettings {
    fn default() -> Self {
        Self {
            worker_prefix: "nots_worker".to_string(),
            worker_labels: HashMap::from([("nots".to_string(), "worker".to_string())]),
        }
    }
}

pub struct DockerRuntime {
    client: bollard::Docker,
    settings: DockerBackendSettings,
}

#[async_trait]
impl NotsRuntime for DockerRuntime {
    async fn worker_create(&self, worker: CreateWorker) -> Result<()> {
        let name = format!("{}-{}", self.settings.worker_prefix, worker.worker_id);

        let WorkerRuntimeOptions::Docker(opt) = worker.runtime_options else {
            bail!("Invalid runtime options for runtime");
        };

        match opt {
            DockerRuntimeOptions::Standalone { image, tag } => {
                unimplemented!("Standalone docker workers are not yet supported")
            }
            DockerRuntimeOptions::Custom { image, tag } => {
                self.create_worker_container(&name, &image, &tag, None)
                    .await?;
            }
            DockerRuntimeOptions::Bun {
                version,
                global_cache,
            } => {
                let image = format!("ghcr.io/explodingcamera/nots-worker:bun-{}", version);
                let tag = "latest".to_string();
                let binds = if global_cache {
                    Some(vec!["nots_bun_cache:/tmp/bun-cache:rw".to_string()])
                } else {
                    None
                };

                self.create_worker_container(&name, &image, &tag, binds)
                    .await?;
            }
        };

        Ok(())
    }

    async fn workers_get(&self) -> Result<HashMap<String, WorkerStatus>> {
        let all = self.get_all_worker_containers().await?;
        let mut workers = HashMap::new();
        for c in all {
            let Some(id) = c.id else {
                continue;
            };

            let status = string_to_status(c.status);
            workers.insert(id, status);
        }
        Ok(workers)
    }

    async fn worker_state(&self, id: &str) -> Result<WorkerState> {
        let container = self.client.inspect_container(id, None).await?;
        Ok(inspect_to_state(container))
    }

    async fn worker_remove(&self, id: &str) -> Result<()> {
        self.client
            .remove_container(
                id,
                Some(RemoveContainerOptions {
                    force: true,
                    ..Default::default()
                }),
            )
            .await?;
        Ok(())
    }
}

impl DockerRuntime {
    pub fn try_new(settings: DockerBackendSettings) -> Result<Self> {
        let client = bollard::Docker::connect_with_local_defaults()?;

        Ok(Self { client, settings })
    }

    async fn get_all_worker_containers(&self) -> Result<Vec<ContainerSummary>> {
        let mut filters = HashMap::new();
        filters.insert("label".to_string(), vec!["nots=worker".to_string()]);

        let options: ListContainersOptions<String> = ListContainersOptions {
            all: true,
            filters,
            ..Default::default()
        };

        let containers = self.client.list_containers(Some(options)).await?;
        Ok(containers)
    }

    async fn start_container(&self, id: &str) -> Result<()> {
        self.client
            .start_container(id, None::<StartContainerOptions<String>>)
            .await?;

        Ok(())
    }

    async fn create_worker_container(
        &self,
        name: &str,
        image: &str,
        tag: &str,
        binds: Option<Vec<String>>,
    ) -> Result<String> {
        let mut binds = binds.unwrap_or_default();
        binds.push("nots_worker_api:/tmp/nots:rw".to_string());

        let host_config = bollard::models::HostConfig {
            binds: Some(binds),
            ..Default::default()
        };

        let c = self
            .client
            .create_container(
                Some(CreateContainerOptions {
                    name,
                    platform: None,
                }),
                bollard::container::Config {
                    image: Some("hello-world"),
                    cmd: Some(vec!["echo", "hello world"]),
                    host_config: Some(host_config),
                    ..Default::default()
                },
            )
            .await?;

        Ok(c.id)
    }

    async fn create_network(&self, name: &str, external_access: bool) -> Result<()> {
        self.client
            .create_network(CreateNetworkOptions {
                name,
                check_duplicate: true,
                driver: "bridge",
                internal: !external_access,
                attachable: false,
                ingress: false,
                ipam: Ipam::default(),
                enable_ipv6: false,
                options: HashMap::new(),
                labels: HashMap::new(),
            })
            .await?;

        Ok(())
    }
}

fn inspect_to_state(container: bollard::service::ContainerInspectResponse) -> WorkerState {
    let restart_count = container.restart_count.map(|s| s as u64);
    let state = container.state.unwrap_or_default();
    let status = state
        .status
        .map(|s| s.as_ref().to_string())
        .unwrap_or_default();

    WorkerState {
        status: string_to_status(Some(status)),
        restart_count,
    }
}

fn string_to_status(s: Option<String>) -> WorkerStatus {
    match s.unwrap_or_default().as_str() {
        "created" => WorkerStatus::Created,
        "running" => WorkerStatus::Running,
        "paused" => WorkerStatus::Paused,
        "restarting" => WorkerStatus::Restarting,
        "removing" => WorkerStatus::Removing,
        "exited" => WorkerStatus::Exited,
        "dead" => WorkerStatus::Dead,
        _ => WorkerStatus::Dead,
    }
}
