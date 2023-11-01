use std::{path::Path, str::FromStr};

use color_eyre::eyre::{eyre, Context, Result};
use git2::{Cred, CredentialType, RemoteCallbacks};
use hyper::Uri;
use zeroize::Zeroizing;

static TEST_SSH_KEY: &str = r#"-----BEGIN OPENSSH PRIVATE KEY-----
b3BlbnNzaC1rZXktdjEAAAAABG5vbmUAAAAEbm9uZQAAAAAAAAABAAAAMwAAAAtzc2gtZW
QyNTUxOQAAACBTUMETLtiMLsZQVMGu0w15ULrRX20JGg5wx3EO3c4QRAAAAJBacDNZWnAz
WQAAAAtzc2gtZWQyNTUxOQAAACBTUMETLtiMLsZQVMGu0w15ULrRX20JGg5wx3EO3c4QRA
AAAEAlnmYKC4xg88Z5YvFRarPJFvGxvYucIa6xkNo33mSZP1NQwRMu2IwuxlBUwa7TDXlQ
utFfbQkaDnDHcQ7dzhBEAAAADWhlbnJ5QHRlbXBvcmE=
-----END OPENSSH PRIVATE KEY-----"#;

#[derive(Debug, Clone)]
pub enum GitCreds {
    Https { username: String, password: String },
    Ssh { ssh_private_key: Zeroizing<String> },
}

pub struct Auth {
    credentials: Option<GitCreds>,
}

impl Auth {
    pub fn new(credentials: Option<GitCreds>) -> Self {
        Self { credentials }
    }

    pub fn anonymous() -> Self {
        Self { credentials: None }
    }

    pub fn credential_callback(
        &self,
    ) -> impl Fn(&str, Option<&str>, CredentialType) -> Result<Cred, git2::Error> + '_ {
        move |url, username_from_url, _allowed_types| match &self.credentials {
            None => Cred::default(),
            Some(GitCreds::Https { username, password }) => {
                Cred::userpass_plaintext(username, password)
            }
            Some(GitCreds::Ssh { ssh_private_key }) => Cred::ssh_key_from_memory(
                username_from_url.unwrap_or("git"),
                None,
                ssh_private_key,
                None,
            ),
        }
    }

    pub fn to_url(&self, url: &str) -> Result<String> {
        match self.credentials {
            None => to_git_https_url(url),
            Some(GitCreds::Https { .. }) => to_git_https_url(url),
            Some(GitCreds::Ssh { .. }) => to_git_ssh_url(url),
        }
    }
}

pub async fn clone(repo: &str, branch: Option<&str>, auth: Auth, out: &str) -> Result<()> {
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(auth.credential_callback());

    let mut fo = git2::FetchOptions::new();
    fo.remote_callbacks(callbacks);
    fo.download_tags(git2::AutotagOption::None);

    let mut builder = git2::build::RepoBuilder::new();
    builder.fetch_options(fo);
    let url = auth.to_url(repo)?;

    if let Some(branch) = branch {
        builder.branch(branch);
    }

    println!("cloning {} into {}", url, out);
    builder
        .clone(&url, Path::new(out))
        .context("failed to clone")?;

    Ok(())
}

pub fn to_git_https_url(url: &str) -> Result<String> {
    let uri = Uri::from_str(url)?;
    let host = uri
        .host()
        .ok_or_else(|| eyre!("invalid url: {}", uri))?
        .to_string();

    let path = uri.path().to_string();
    let path = path.strip_prefix('/').unwrap_or(&path);
    Ok(format!("https://{}/{}", host, path))
}

pub fn to_git_ssh_url(url: &str) -> Result<String> {
    let uri = Uri::from_str(url)?;
    let host = uri
        .host()
        .ok_or_else(|| eyre!("invalid url: {}", uri))?
        .to_string();

    let path = uri.path().to_string();
    let path = path.strip_prefix('/').unwrap_or(&path);
    let port = uri.port_u16().unwrap_or(22);
    Ok(format!("ssh://git@{}:{}/{}", host, port, path))
}
