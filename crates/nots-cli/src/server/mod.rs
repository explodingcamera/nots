#[cfg(feature = "docker")]
pub mod docker;
#[cfg(feature = "docker")]
pub use docker::DockerBackend;

#[cfg(feature = "systemd")]
mod systemd;

pub trait ServerBackend {
    fn create(&self) -> Result<(), String>;
    fn stop(&self) -> Result<(), String>;
    fn start(&self) -> Result<(), String>;
    fn remove(&self) -> Result<(), String>;
    fn update(&self) -> Result<(), String>;
    fn restart(&self) -> Result<(), String>;
    fn status(&self) -> Result<(), String>;
}
