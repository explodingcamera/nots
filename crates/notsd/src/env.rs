pub struct Env {
    pub nots_secret: String,
    pub nots_worker_bind: String,
    pub nots_api_bind: String,
    pub nots_backend: String,
}

pub fn new() -> Env {
    Env::new()
}

impl Env {
    pub fn new() -> Self {
        use std::env;

        let nots_secret =
            env::var("NOTS_SECRET").unwrap_or_else(|_| match cfg!(debug_assertions) {
                true => "00000000000000000000000000000000".to_string(),
                false => panic!("NOTS_SECRET must be set"),
            });

        Self {
            nots_secret,
            nots_backend: env::var("NOTS_BACKEND").unwrap_or("docker".to_string()),
            nots_api_bind: env::var("NOTS_API_BIND").unwrap_or("/tmp/nots/api.sock".to_string()),
            nots_worker_bind: env::var("NOTS_WORKER_BIND")
                .unwrap_or("/tmp/nots/worker/worker.sock".to_string()),
        }
    }
}
