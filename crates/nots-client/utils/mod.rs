// https://api.github.com/repos/explodingcamera/nots/tags

pub use semver::Version;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct Release {
    tag_name: String,
    draft: bool,
    prerelease: bool,
    published_at: String,
    body: String,
}

mod http;
pub use http::*;
