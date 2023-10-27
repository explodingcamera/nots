use std::collections::HashMap;

use bollard::{container::CreateContainerOptions, network::CreateNetworkOptions, service::Ipam};
use color_eyre::eyre::Result;

pub struct Docker {
    client: bollard::Docker,
}

impl Docker {
    fn new() -> Self {
        let client = bollard::Docker::connect_with_local_defaults().unwrap();

        Self { client }
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
