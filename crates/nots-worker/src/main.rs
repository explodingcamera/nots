use color_eyre::eyre::Result;
use hyper::{Body, Response};
use hyperlocal::UnixConnector;
use serde::Deserialize;

mod utils;

#[derive(Debug, Deserialize)]
struct WorkerSettings {
    pub port: u16,
}

#[derive(Debug, Deserialize)]
struct NotsResponse {
    settings: WorkerSettings,
}

const SOCKET_PATH: &str = "/tmp/nots/worker.sock";

struct State {
    pub settings: Option<WorkerSettings>,
    pub client: hyper::Client<UnixConnector>,
    pub app_id: String,
}

impl State {
    async fn req(&self, uri: &str, method: &str, body: Option<Body>) -> Result<Response<Body>> {
        let addr: hyper::Uri = hyperlocal::Uri::new(SOCKET_PATH, uri).into();
        let req = hyper::Request::builder()
            .method(method)
            .header("x-nots-app", &self.app_id)
            .uri(addr)
            .body(body.unwrap_or(Body::empty()))?;

        let res = self.client.request(req).await?;
        Ok(res)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    utils::install_tracing();

    let app_id =
        std::env::var("NOTS_APPLICATION_ID").unwrap_or_else(|_| match cfg!(debug_assertions) {
            true => "test".to_owned(),
            false => panic!("NOTS_APPLICATION_ID needs to be set"),
        });

    let mut state = State {
        settings: None,
        client: hyper::Client::builder().build(UnixConnector),
        app_id,
    };

    loop {
        run(&mut state).await?;
        tokio::time::sleep(tokio::time::Duration::from_secs(20)).await;
    }
}

async fn run(state: &mut State) -> Result<()> {
    let res = state.req("", "GET", None).await?;
    let body = utils::parse_body::<NotsResponse>(res).await?;
    println!("{:?}", body);
    Ok(())
}
