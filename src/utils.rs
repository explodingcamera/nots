use std::net::SocketAddr;

use axum::http::HeaderValue;
use hyper::HeaderMap;

pub fn add_x_forwarded_for(headers: &mut HeaderMap<HeaderValue>, addr: SocketAddr) {
    let client_ip = addr.ip().to_string();
    if let Some(existing_header) = headers.get("X-Forwarded-For") {
        // Append the client IP address if the header already exists
        let updated_header = format!("{}, {}", existing_header.to_str().unwrap_or(""), client_ip);
        headers.insert("X-Forwarded-For", updated_header.parse().unwrap());
    } else {
        // Add a new header if it doesn't already exist
        headers.insert("X-Forwarded-For", client_ip.parse().unwrap());
    }
}

pub fn remove_hop_by_hop_headers(headers: &mut HeaderMap<HeaderValue>) {
    headers.remove("connection");
    headers.remove("keep-alive");
    headers.remove("proxy-authenticate");
    headers.remove("proxy-authorization");
    headers.remove("te");
    headers.remove("trailers");
    headers.remove("transfer-encoding");
    headers.remove("upgrade");
}

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
