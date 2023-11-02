#![allow(unused)]

use clap::Parser;
use color_eyre::eyre::Result;
use commands::{Cli, Commands};

mod api;
mod commands;
mod server;
mod utils;

pub struct State {
    pub api: api::Client,
    pub global_args: Cli,
}

#[tokio::main]
async fn main() -> Result<()> {
    nots_core::install_tracing(None);
    color_eyre::install()?;

    let args = Cli::parse();
    let state = State {
        api: api::Client::new(api::TransportSettings::Unix(api::unix::UnixSettings {
            path: "/tmp/nots/api.sock".into(),
        })),
        global_args: args,
    };

    match state.global_args.command.clone() {
        Commands::Server { command } => commands::server::run(&command, state).await?,
    };

    Ok(())
}
