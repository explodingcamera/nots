use clap::{Args, Subcommand};

pub fn run(args: Server) {
    println!("{:?}", args);
}

#[derive(Debug, Args)]
pub struct Server {
    #[command(subcommand)]
    command: Option<ServerCommand>,
}

#[derive(Debug, Subcommand)]
pub enum ServerCommand {
    Init { port: Option<u16> },
}
