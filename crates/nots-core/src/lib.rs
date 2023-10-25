mod utils;
pub use utils::*;

pub mod worker {
    use std::collections::HashMap;

    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize)]
    pub struct WorkerSettings {
        pub port: u16,
        pub command: Option<String>,
        pub main: Option<String>,
        pub env: HashMap<String, String>,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct WorkerRegisterResponse {
        pub settings: WorkerSettings,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct WorkerHeartbeatResponse {
        pub ok: bool,
    }
}
