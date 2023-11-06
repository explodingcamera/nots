// https://api.github.com/repos/explodingcamera/nots/tags

use std::str::FromStr;

use color_eyre::eyre::Result;
use hyper::{Client, Uri};
use semver::Version;

#[derive(serde::Deserialize, serde::Serialize)]
struct Release {
    tag_name: String,
    draft: bool,
    prerelease: bool,
    published_at: String,
    body: String,
}

pub async fn get_version_by_prefix(
    repo: &str,
    prefix: &str,
    include_prerelease: bool,
) -> Result<Vec<Version>> {
    let client = Client::new();
    let url = Uri::from_str(&format!("https://api.github.com/repos/{}/releases", repo))?;
    let mut res = client.get(url).await?;
    let body = hyper::body::to_bytes(res.body_mut()).await?;
    let releases: Vec<Release> = serde_json::from_slice(&body)?;

    let mut versions: Vec<Version> = releases
        .iter()
        .filter(|t| t.tag_name.starts_with(prefix))
        .map(|t| t.tag_name.replace(prefix, "").replace("v", ""))
        .map(|t| Version::parse(&t).unwrap())
        .filter(|v| include_prerelease || !v.pre.is_empty())
        .collect();

    versions.sort();
    Ok(versions)
}
