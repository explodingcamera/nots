use std::{io::Write, process::exit};

use clap::Args;
use color_eyre::eyre::{bail, ContextCompat, Result};
use colored::Colorize;
use futures::StreamExt;
use hyper::body::HttpBody;
use nots_client::utils::{create_https_client, get_version_by_prefix, Version};
use spinoff::{spinners, Spinner};
use tokio::process::Command;

use crate::State;

#[derive(Debug, Clone, Args)]
pub struct UpgradeCommand {
    #[clap()]
    /// Upgrade to a specific version, e.g. `nots upgrade 0.1.0`
    pub version: Option<String>,

    #[clap(long, short)]
    /// Force upgrade to requested version
    pub force: bool,

    #[clap(long, short)]
    /// Include prerelease versions
    pub prerelease: bool,
}

static REPO: &str = "explodingcamera/nots";

pub async fn run(args: &UpgradeCommand, state: State) -> Result<()> {
    if cfg!(windows) {
        bail!("Upgrade is not supported on Windows yet");
    }

    let current = Version::parse(CURRENT_VERSION)?;
    let mut version_spinner = Spinner::new(
        spinners::Dots,
        "Checking for updates...",
        spinoff::Color::Green,
    );
    let versions = get_version_by_prefix(REPO, "nots-cli", args.prerelease).await?;
    version_spinner.clear();

    let requested_version = if let Some(version) = &args.version {
        let Ok(version) = Version::parse(version) else {
            println!("{}", "Invalid version specified".bright_red());
            println!(
                "{}\n{}",
                format!("  Version: {}", version.bright_black()).white(),
                format!("  Format:  {}", "major.minor.patch".bright_black()).white()
            );
            exit(-1)
        };

        if !versions.contains(&version) {
            println!("{}", format!("Version {} not found", version).bright_red());
            exit(-1)
        }

        version
    } else {
        versions.last().context("No versions found")?.clone()
    };

    if !args.force {
        if current == requested_version {
            println!(
                "{} {} {}",
                "Congratulations!".bright_green().bold(),
                "You are already on the latest version".bright_white(),
                format!("(v{})", current).bright_black()
            );
            return Ok(());
        }
        if current > requested_version {
            println!(
                "You are on a newer version than the requested version: {} > {}",
                current, requested_version
            );
            return Ok(());
        }
    }

    #[cfg(unix)]
    install_version(&requested_version.to_string()).await?;

    Ok(())
}

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
static TARGET: &str = "x86_64-unknown-linux";
#[cfg(all(target_os = "linux", target_arch = "aarch64"))]
static TARGET: &str = "aarch64-unknown-linux";
#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
static TARGET: &str = "x86_64-apple-darwin";
#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
static TARGET: &str = "aarch64-apple-darwin";

#[cfg(unix)]
async fn install_version(version: &str) -> Result<()> {
    let client = create_https_client(false);
    let uri = format!(
        "https://github.com/{}/releases/download/nots-cli-v{}/nots-cli-{}.tar.gz",
        REPO, version, TARGET
    );

    let mut res = reqwest::get(&uri).await?;
    let temp_dir = tempfile::tempdir()?;
    let path = &temp_dir.path().join("nots-cli.tar.gz");

    println!(
        "{}{}",
        "Installing nots".green().bold(),
        format!(" v{}", version.bright_white()).bold()
    );
    let mut dl_spinner = Spinner::new(spinners::Dots, "Downloading...", spinoff::Color::Green);

    {
        let mut file = std::fs::File::create(&path)?;
        let mut body = res.bytes_stream();
        while let Some(chunk) = body.next().await {
            file.write_all(&chunk?)?;
        }
    }

    dl_spinner.clear();

    let nots_location = if cfg!(debug_assertions) {
        "~/local/bin".into()
    } else {
        std::env::current_exe()?.parent().unwrap().to_path_buf()
    };

    let res = inquire::Confirm::new(&format!("Install nots to {}?", nots_location.display()))
        .with_default(true)
        .prompt()?;

    if !res {
        println!("Aborting installation");
        return Ok(());
    }

    if !Command::new("tar")
        .arg("-xzf")
        .arg(&path)
        .arg("-C")
        .arg(&path.parent().unwrap())
        .spawn()?
        .wait()
        .await?
        .success()
    {
        bail!("Could not extract downloaded file. Ensure that tar is installed and in your PATH");
    }

    let mut install_spinner = Spinner::new(spinners::Dots, "Installing...", spinoff::Color::Green);

    let mut cmd = Command::new("chmod")
        .arg("+x")
        .arg(&path.parent().unwrap().join("nots-cli"))
        .spawn()?;

    if !cmd.wait().await?.success() {
        bail!("Could not make nots-cli executable");
    }

    let mut cmd = Command::new("mv")
        .arg(&path.parent().unwrap().join("nots-cli"))
        .arg(&nots_location.join("nots"))
        .spawn()?;
    if !cmd.wait().await?.success() {
        bail!("Could not move nots-cli to {}", nots_location.display());
    }
    install_spinner.clear();

    println!(
        "{}",
        format!("Successfully installed nots v{}", version)
            .green()
            .bold()
    );

    Ok(())
}

const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");
