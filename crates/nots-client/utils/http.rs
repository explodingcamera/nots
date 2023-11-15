use color_eyre::eyre::Result;
use reqwest::header::HeaderMap;
use semver::Version;

use super::Release;

const GITHUB_API: &str = "https://api.github.com";
const GITHUB: &str = "https://github.com";

pub fn default_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("User-Agent", "nots".parse().unwrap());
    headers
}

#[cfg(feature = "tls")]
pub fn create_https_client(allow_insecure: bool) -> Result<reqwest::Client> {
    let client = reqwest::Client::builder()
        .https_only(true)
        .default_headers(default_headers())
        .build()?;

    Ok(client)
}

pub fn create_http_only_client() -> Result<reqwest::Client> {
    let client = reqwest::Client::builder()
        .http1_only()
        .default_headers(default_headers())
        .build()?;

    Ok(client)
}

#[cfg(feature = "tls")]
pub async fn get_github_version_by_prefix(repo: &str, prefix: &str, include_prerelease: bool) -> Result<Vec<Version>> {
    let client = create_https_client(false)?;
    let uri = format!("{GITHUB_API}/repos/{}/releases", repo);
    let releases = client.get(&uri).send().await?.json::<Vec<Release>>().await?;

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

#[cfg(feature = "tls")]
/// Download a github release artifact
/// Requires the format tag:
pub async fn download_github_release_artifact(
    repo: &str,
    version: &str,
    tag_prefix: &str,
    filename: &str,
    save_to: &std::path::Path,
) -> Result<std::path::PathBuf> {
    use futures::StreamExt;
    use tokio::io::AsyncWriteExt;

    let path = save_to.join(filename);
    let client = create_https_client(false);
    let uri = format!("{GITHUB}/{repo}/releases/download/{tag_prefix}{version}/{filename}");

    let mut res = reqwest::get(&uri).await?;

    let mut file = tokio::fs::File::create(&path).await?;
    let mut body = res.bytes_stream();
    while let Some(chunk) = body.next().await {
        file.write_all(&chunk?).await?;
    }

    Ok(path)
}
