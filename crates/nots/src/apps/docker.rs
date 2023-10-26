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
        self.client
            .create_container(
                None::<CreateContainerOptions<String>>,
                bollard::container::Config {
                    image: Some("hello-world"),
                    cmd: Some(vec!["echo", "hello world"]),
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
