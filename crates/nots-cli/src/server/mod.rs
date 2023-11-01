#[cfg(feature = "docker")]
mod docker;

#[cfg(feature = "systemd")]
mod systemd;

pub trait Server {
    fn create(&self) -> Result<(), String>;
    fn stop(&self) -> Result<(), String>;
    fn start(&self) -> Result<(), String>;
    fn remove(&self) -> Result<(), String>;
    fn update(&self) -> Result<(), String>;
    fn restart(&self) -> Result<(), String>;
    fn status(&self) -> Result<(), String>;
}
