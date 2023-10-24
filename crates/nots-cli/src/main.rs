use args::Cli;
use clap::Parser;
use color_eyre::eyre::Result;
mod args;
mod commands;
mod utils;

fn main() -> Result<()> {
    utils::install_tracing();
    color_eyre::install()?;

    let args = Cli::parse();

    use args::Commands::*;
    match args.command {
        Server(c) => commands::server::run(c),
    };

    Ok(())
}
