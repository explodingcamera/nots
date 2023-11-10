// https://api.github.com/repos/explodingcamera/nots/tags

use color_eyre::eyre::Result;
use hyper::Client;
pub use semver::Version;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct Release {
    tag_name: String,
    draft: bool,
    prerelease: bool,
    published_at: String,
    body: String,
}

pub fn create_https_client(
    allow_insecure: bool,
) -> hyper::Client<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>> {
    let https = hyper_rustls::HttpsConnectorBuilder::new().with_native_roots();

    let https = if allow_insecure {
        https.https_only()
    } else {
        https.https_or_http()
    }
    .enable_http1()
    .build();

    Client::builder().build(https)
}

pub async fn get_version_by_prefix(
    repo: &str,
    prefix: &str,
    include_prerelease: bool,
) -> Result<Vec<Version>> {
    let req = hyper::Request::builder()
        .method("GET")
        .uri(format!("https://api.github.com/repos/{}/releases", repo))
        .header("User-Agent", "nots")
        .body((hyper::Body::empty()))?;

    let mut res = create_https_client(false).request(req).await?;
    let body = hyper::body::to_bytes(res.body_mut()).await?;
    let releases: Vec<Release> = serde_json::from_slice(&body)?;

    let mut versions: Vec<Version> = releases
        .iter()
        .filter(|t| t.tag_name.starts_with(prefix))
        .map(|t| t.tag_name.replace(&format!("{}-v", prefix), ""))
        .map(|t| Version::parse(&t).unwrap())
        .filter(|v| !include_prerelease || !v.pre.is_empty())
        .collect();

    versions.sort_by(Version::cmp_precedence);
    Ok(versions)
}
