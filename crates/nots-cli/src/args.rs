use clap::{
    builder::{styling, Styles},
    Parser, Subcommand,
};

use crate::commands;

fn get_styles() -> Styles {
    styling::Styles::styled()
        .header(styling::AnsiColor::Green.on_default() | styling::Effects::BOLD)
        .usage(styling::AnsiColor::Green.on_default() | styling::Effects::BOLD)
        .literal(styling::AnsiColor::Blue.on_default() | styling::Effects::BOLD)
        .placeholder(styling::AnsiColor::Cyan.on_default())
}

#[derive(Debug, Parser)]
#[command(name = "nots")]
#[command(
    styles =  get_styles(),
    author = "Henry Gressmann <mail@henrygressmann.de>",
    version = env!("CARGO_PKG_VERSION"),
    about = "CLI for the nots server",
    long_about = None
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Server(commands::server::Server),
}
