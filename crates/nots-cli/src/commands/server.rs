use crate::{
    server::{DockerBackend, ServerBackend},
    State,
};
use clap::Subcommand;
use color_eyre::eyre::Result;

pub async fn run(args: &ServerCommand, state: State) -> Result<()> {
    let server = Server { state };
    match args {
        ServerCommand::Init(args) => server.init(args).await,
        ServerCommand::Status => server.status().await,
        ServerCommand::Uninstall => server.uninstall().await,
        ServerCommand::Upgrade => server.upgrade().await,
    }
}

struct Server {
    state: State,
}
impl Server {
    async fn init(&self, args: &InitCommand) -> Result<()> {
        println!("{:?}", args);
        Ok(())
    }

    async fn status(&self) -> Result<()> {
        Ok(())
    }

    async fn uninstall(&self) -> Result<()> {
        Ok(())
    }

    async fn upgrade(&self) -> Result<()> {
        Ok(())
    }

    fn get_backend(&self) -> Result<Box<dyn ServerBackend>> {
        Ok(Box::new(DockerBackend::default()))
    }
}

#[derive(Debug, Subcommand, Clone)]
pub enum ServerCommand {
    Init(InitCommand),
    Status,
    Uninstall,
    Upgrade,
}

#[derive(Debug, clap::Args, Clone)]
pub struct InitCommand {
    port: Option<u16>,
}
