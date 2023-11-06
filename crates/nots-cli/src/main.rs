#![allow(dead_code)]
#![allow(unused)]
#![warn(unused_imports)]

use clap::Parser;
use color_eyre::eyre::Result;
use commands::{Cli, Commands};
use nots_client::{unix::UnixSettings, Client, TransportSettings};

mod commands;
mod server;
mod utils;

pub struct State {
    pub client: Client,
    pub global_args: Cli,
}

#[tokio::main]
async fn main() -> Result<()> {
    nots_client::install_tracing(None);
    color_eyre::install()?;

    let args = Cli::parse();
    let state = State {
        client: Client::new(TransportSettings::Unix(UnixSettings {
            path: "/tmp/nots/api.sock".into(),
        })),
        global_args: args,
    };

    match state.global_args.command.clone() {
        Commands::Server { command } => commands::server::run(&command, state).await?,
        Commands::App { command } => commands::app::run(&command, state).await?,
    };

    Ok(())
}
