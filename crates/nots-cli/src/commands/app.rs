use crate::State;
use clap::Subcommand;
use color_eyre::eyre::Result;

pub async fn run(args: &AppCommand, state: State) -> Result<()> {
    let app = App(state);
    match args {
        AppCommand::Create => app.create().await,
        AppCommand::List => app.list().await,
        AppCommand::Edit { name } => app.edit(name).await,
    }
}

struct App(State);

#[derive(Debug, Subcommand, Clone)]
pub enum AppCommand {
    Create,
    List,
    Edit {
        #[clap(short, long)]
        name: String,
    },
}

impl App {
    async fn create(&self) -> Result<()> {
        Ok(())
    }

    async fn list(&self) -> Result<()> {
        Ok(())
    }

    async fn edit(&self, name: &str) -> Result<()> {
        Ok(())
    }
}
