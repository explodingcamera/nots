pub(crate) fn install_tracing() {
    use tracing_error::ErrorLayer;
    use tracing_subscriber::filter::LevelFilter;
    use tracing_subscriber::prelude::*;

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .compact()
                .with_filter(LevelFilter::INFO),
        )
        .with(ErrorLayer::default())
        .init();
}
