use std::collections::HashMap;

use bollard::{container::CreateContainerOptions, network::CreateNetworkOptions, service::Ipam};
use color_eyre::eyre::Result;

use crate::{scheduler::ProcessBackend, state::AppState};

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

pub struct DockerBackend {
    client: bollard::Docker,
    state: AppState,
    settings: DockerBackendSettings,
}

impl ProcessBackend for DockerBackend {
    fn worker_create(&self, worker: crate::scheduler::CreateWorker) -> Result<()> {
        todo!()
    }

    fn workers_get(&self) -> Result<()> {
        todo!()
    }

    fn worker_get(&self, id: &str) -> Result<()> {
        todo!()
    }

    fn worker_remove(&self, id: &str) -> Result<()> {
        todo!()
    }

    fn worker_update(&self) -> Result<()> {
        todo!()
    }

    fn worker_restart(&self, id: &str) -> Result<()> {
        todo!()
    }

    fn worker_status(&self, id: &str) -> Result<()> {
        todo!()
    }
}

impl DockerBackend {
    pub fn try_new(state: AppState, settings: DockerBackendSettings) -> Result<Self> {
        let client = bollard::Docker::connect_with_local_defaults()?;

        Ok(Self {
            client,
            state,
            settings,
        })
    }

    async fn create_worker_container(&self, name: &str, image: &str, tag: &str) -> Result<()> {
        let binds = Some(vec!["nots_worker_api:/tmp/nots:rw".to_string()]);
        let host_config = bollard::models::HostConfig {
            binds,
            ..Default::default()
        };

        self.client
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

        Ok(())
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
