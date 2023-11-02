use std::path::PathBuf;

use color_eyre::eyre::Result;
use tokio::net::UnixStream;

use super::Transport;

pub struct UnixSettings {
    pub path: PathBuf,
}

pub struct UnixTransport {
    socket: UnixStream,
}

impl UnixTransport {
    pub async fn connect(settings: &UnixSettings) -> Result<Self> {
        let path = &settings.path;
        let socket = UnixStream::connect(path).await?;
        Ok(Self { socket })
    }
}

impl Transport for UnixTransport {}
