use clap::{
  builder::{styling, Styles},
  Parser, Subcommand, Args,
};

pub(crate) mod server;
#[derive(Debug, Parser)]
#[command(name = "nots")]
#[command( 
  styles =  get_styles(),
  author = "Henry Gressmann <mail@henrygressmann.de>",
  version = env!("CARGO_PKG_VERSION"),
  about = "Client for connecting to and starting a nots daemon Server",
  long_about = None
)]
pub struct Cli {
  #[command(subcommand)]
  pub command: Commands,
}

#[derive(Debug, Subcommand, Clone)]
pub enum Commands {
  #[command(arg_required_else_help(true))]
  Server {
      #[command(subcommand)]
      command: server::ServerCommand,
  },
}

fn get_styles() -> Styles {
  styling::Styles::styled()
      .header(styling::AnsiColor::Green.on_default() | styling::Effects::BOLD)
      .usage(styling::AnsiColor::Green.on_default() | styling::Effects::BOLD)
      .literal(styling::AnsiColor::Blue.on_default() | styling::Effects::BOLD)
      .placeholder(styling::AnsiColor::Cyan.on_default())
}