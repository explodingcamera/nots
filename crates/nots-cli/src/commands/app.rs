use crate::{server::DockerBackend, State};
use clap::Subcommand;
use color_eyre::eyre::Result;

pub async fn run(args: &AppCommand, state: State) -> Result<()> {
    let app = App(state);
    match args {
        AppCommand::Create => app.create().await,
    }
}

struct App(State);

#[derive(Debug, Subcommand, Clone)]
pub enum AppCommand {
    Create,
}

impl App {
    async fn create(&self) -> Result<()> {
        Ok(())
    }
}
