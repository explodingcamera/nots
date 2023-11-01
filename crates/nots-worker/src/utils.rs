use color_eyre::eyre::Result;
use hyper::{Body, Response};
use serde::de::DeserializeOwned;

pub async fn parse_body<T>(res: Response<Body>) -> Result<T>
where
    T: DeserializeOwned,
{
    let body = hyper::body::to_bytes(res.into_body()).await?;
    let settings = serde_json::from_slice(&body)?;
    Ok(settings)
}
