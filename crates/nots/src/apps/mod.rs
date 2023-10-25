use nots_core::app::AppSettings;
mod host;

#[cfg(feature = "git")]
mod docker;

pub struct RunningApp {
    pub settings: AppSettings,
    pub settings_updated_at: time::OffsetDateTime,

    pub container_id: Option<String>,
    pub process_id: Option<u32>,
}
