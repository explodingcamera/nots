use std::process::Command;

use crate::{
    server::{DockerBackend, ServerBackend},
    State,
};
use clap::Subcommand;
use color_eyre::{eyre::Result, owo_colors::OwoColorize};
use colored::*;
use inquire::Confirm;
use nots_client::api::ServerStatus;

pub async fn run(args: &ServerCommand, state: State) -> Result<()> {
    let server = Server { state };

    match args {
        ServerCommand::Init(args) => server.init(args).await,
        ServerCommand::Status => server.status().await,
        ServerCommand::Uninstall => server.uninstall().await,
        ServerCommand::Upgrade => server.upgrade().await,
        ServerCommand::Ps => server.ps().await,
    }
}

struct Server {
    state: State,
}

impl Server {
    async fn init(&self, args: &InitCommand) -> Result<()> {
        let backend = self.get_backend()?;
        if !backend.is_supported().await {
            println!(
                "{}\n{}",
                "Could not connect to docker daemon".red(),
                "Please make sure docker is installed and running".yellow(),
            );

            return Ok(());
        }

        if backend.get().await?.is_some() {
            println!("{}", "Nots server is already running".green());
            return Ok(());
        }

        println!(
            "{}\n{}\n{}",
            "Welcome to the nots server setup".blue().bold(),
            "> This will create a new docker container called `notsd`".blue(),
            "> This deamon will run in the background, manage your apps and proxy requests to them"
                .blue(),
        );

        if cfg!(target_family = "unix") {
            // check if a nots group exists
            let output = Command::new("getent")
                .arg("group")
                .arg("nots")
                .output()
                .expect("failed to execute `getent group nots`");
            if output.status.success() {
                println!(
                    "{}",
                    "The `nots` group already exists, skipping group creation".yellow()
                );
            } else {
                println!("\n{}", "Creating the `nots` group...".green().bold(),);

                runas::Command::new("groupadd")
                    .arg("nots")
                    .status()
                    .expect("failed to create nots group");

                println!(
                    "\n{}",
                    "Successfully created the `nots` group".green().bold(),
                );

                if Confirm::new("Do you want to join the `nots` group now?")
                    .with_default(true)
                    .prompt()?
                {
                    runas::Command::new("usermod")
                        .arg("-aG")
                        .arg("nots")
                        .arg(&whoami::username())
                        .status()?;

                    println!(
                        "{}",
                        "You will need to log out and log back in for this to take effect".green()
                    );
                } else {
                    println!(
                        "{}",
                        "You can run `usermod -aG nots $USER` to join the group later".green()
                    );
                }
            }
        }

        println!("");
        // let ssl = Confirm::new("Do you want nots to handle SSL termination?")
        //     .with_default(false)
        //     .prompt()?;

        let port: u16 = inquire::CustomType::new("Which port should the container listen on")
            .with_default(8080)
            .prompt()?;

        println!(
            "\n{}{}\n",
            "Final Configuration:\n".green().bold(),
            format!("  Port: {}", port.bright_black()).white(),
        );

        let ans = Confirm::new("Do you want to continue?")
            .with_default(true)
            .prompt();

        if !ans? {
            println!("{}", "Aborting".red().bold());
            return Ok(());
        }

        println!("\n{}", "Creating the `notsd` container...".green().bold(),);
        backend.create().await?;

        println!(
            "{} {}",
            "Notsd is now listening to requests on".white().dimmed(),
            format!("http://localhost:{}", port)
                .bright_white()
                .underline(),
        );

        println!(
            "\n{}\n{}{}\n",
            "You can now start by creating a new app with"
                .white()
                .dimmed(),
            "$ ".bright_black(),
            "nots app create".bright_white(),
        );

        Ok(())
    }

    async fn ps(&self) -> Result<()> {
        Ok(())
    }

    async fn status(&self) -> Result<()> {
        let res: ServerStatus = self
            .state
            .client
            .req_json(
                "http://localhost:8080/status",
                "GET",
                None::<String>,
                Some(hyper::HeaderMap::new()),
            )
            .await?;

        println!("{:?}", res);

        Ok(())
    }

    async fn uninstall(&self) -> Result<()> {
        Ok(())
    }

    async fn upgrade(&self) -> Result<()> {
        Ok(())
    }

    fn get_backend(&self) -> Result<Box<dyn ServerBackend>> {
        Ok(Box::<DockerBackend>::default())
    }
}

#[derive(Debug, Subcommand, Clone)]
pub enum ServerCommand {
    Init(InitCommand),
    Ps,
    Status,
    Uninstall,
    Upgrade,
}

#[derive(Debug, clap::Args, Clone)]
pub struct InitCommand {
    port: Option<u16>,
}
