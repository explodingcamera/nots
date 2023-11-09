use std::{process::Command, time::Duration};

use crate::{
    server::{DockerBackend, ServerBackend},
    State,
};
use clap::Subcommand;
use color_eyre::{
    eyre::{ContextCompat, Result},
    owo_colors::OwoColorize,
};
use colored::*;
use inquire::{validator::Validation, Confirm};
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

        if let Some(container) = backend.get().await? {
            println!("{}", "You already have notsd installed".yellow());
            println!(
                "{}",
                format!("  ID: {}", container.id.bright_black()).white()
            );
            println!(
                "{}",
                format!("  Status: {}", container.status.bright_black()).white()
            );
            println!(
                "{}",
                format!("  Runtime: {}", container.runtime.bright_black()).white()
            );

            let ans = Confirm::new("Do you want to remove the existing notsd container?")
                .with_help_message("This will only remove the container, not the attached volumes")
                .with_default(false)
                .prompt()?;

            if ans {
                backend.remove().await?;
            } else {
                println!("{}", "Aborting".red().bold());
                return Ok(());
            }
        }

        println!(
            "{}\n{}\n{}",
            "Welcome to the nots server setup".blue().bold(),
            "> This will create a new docker container called `notsd`".blue(),
            "> This container runs the nots daemon which runs in the background and handles all requests to your apps"
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

        // let ssl = Confirm::new("Do you want nots to handle SSL termination?")
        //     .with_default(false)
        //     .prompt()?;

        println!();
        let interface: String =
            inquire::Text::new("Which interface should the webserver listen on?")
                .with_default("0.0.0.0")
                .with_help_message("e.g. 127.0.0.1 to only listen on localhost")
                .with_validator(|s: &str| {
                    if s.is_empty() {
                        return Ok(Validation::Invalid(
                            "The interface cannot be empty".to_string().into(),
                        ));
                    }

                    let ip = s.parse::<std::net::IpAddr>();
                    if ip.is_err() {
                        return Ok(Validation::Invalid(
                            "The interface must be a valid IP address"
                                .to_string()
                                .into(),
                        ));
                    }

                    Ok(Validation::Valid)
                })
                .prompt()?;

        let port: u16 = inquire::CustomType::new("Which port should the webserver listen on?")
            .with_default(8080)
            .prompt()?;

        let secret: String =
            inquire::Password::new("What should the secret be? (at least 16 characters)")
                .with_display_mode(inquire::PasswordDisplayMode::Masked)
                .without_confirmation()
                .with_help_message("This is used to encrypt secrets and tokens in the database")
                .with_validator(|s: &str| {
                    if s.len() < 16 {
                        Ok(Validation::Invalid(
                            "The secret must be at least 16 characters long"
                                .to_string()
                                .into(),
                        ))
                    } else {
                        Ok(Validation::Valid)
                    }
                })
                .prompt()?;

        println!();
        println!(
            "{}{}{}{}",
            "Final Configuration:\n".green().bold(),
            format!("  Interface: {}\n", interface.bright_black()).white(),
            format!("  Port: {}\n", port.bright_black()).white(),
            format!(
                "  Secret: {}\n",
                str::repeat("*", secret.len()).bright_black()
            )
            .white(),
        );

        let ans = Confirm::new("Continue with the above configuration?")
            .with_default(true)
            .prompt();

        if !ans? {
            println!("{}", "Aborting".red().bold());
            return Ok(());
        }

        println!("\n{}", "Creating the `notsd` container...".green().bold(),);
        backend.create("0.1.5", port, &secret).await?;

        println!(
            "{} {}",
            "Notsd is now listening to requests on".green().bold(),
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
        let (res, parts) = self
            .state
            .client
            .req_json::<_, ServerStatus>(
                "http://localhost:8080/status",
                "GET",
                None::<String>,
                Some(hyper::HeaderMap::new()),
            )
            .await?;

        let powered_by = parts
            .headers
            .get("x-powered-by")
            .context("Could not get server version")?
            .to_str()?;

        let version = powered_by
            .strip_prefix("nots/")
            .context("Could not get server version")?;

        println!(
            "{}",
            format!("Connected to Notsd v{}", version)
                .bright_white()
                .bold()
        );
        let uri = self.state.client.printable_client_uri();
        println!("  Client URI: {}", uri.bright_black().bold()).bright_white();
        println!(
            "  Uptime:     {:?}",
            Duration::from_secs(res.uptime_secs).bright_black().bold()
        );

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
