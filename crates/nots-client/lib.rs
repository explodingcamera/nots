#![allow(dead_code)]
#![allow(unused)]
#![warn(unused_imports)]

#[cfg(feature = "api")]
pub mod api;

#[cfg(feature = "worker")]
pub mod worker;

pub mod models;
pub mod utils;

mod client;
pub use client::*;

pub fn install_tracing(log_level: Option<tracing::Level>) {
    use tracing_error::ErrorLayer;
    use tracing_subscriber::filter::LevelFilter;
    use tracing_subscriber::prelude::*;

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .compact()
                .with_filter(LevelFilter::from(log_level.unwrap_or(tracing::Level::INFO))),
        )
        .with(ErrorLayer::default())
        .init();
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct EncryptedBytes(pub Vec<u8>);
