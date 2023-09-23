use std::io::Read;
use reqwest::Response;
use serde::{de::DeserializeOwned, Deserialize};
use async_trait::async_trait;
use anyhow::{Result, Error};
use serde_json::Value;

async fn parse_body(res: Response) -> Result<String> {
    let bytes = res.bytes().await?;
    let body = String::from_utf8(bytes.bytes().flatten().collect())?;
    Ok(body)
}

#[async_trait]
pub trait FromResponse: Sized {
    async fn from_response(req: Response) -> Result<Self>;
}

#[derive(Debug, Deserialize)]
struct JsonMessage {
    message: String
}

#[derive(Debug)]
pub struct Json<T> (pub T);

#[allow(dead_code)]
fn pretty(s: &str) {
    let v: Value = serde_json::from_str(s).unwrap();
    println!("{}", serde_json::to_string_pretty(&v).unwrap());
}

#[async_trait]
impl<T> FromResponse for Json<T> 
where
    T: DeserializeOwned + Send + Sync
{
    async fn from_response(res: Response) -> Result<Self> {
        let status = res.status();
        let body = parse_body(res).await?;

        // pretty(&body);

        if !status.is_success() {
            let Ok(value) = serde_json::from_str::<JsonMessage>(&body) else {
                return Err(Error::msg(format!("request failed with status: {}", status.as_u16())))
            };
            return Err(Error::msg(format!("request failed with {}: {}", status.as_u16(), value.message)))
        }
        let value = match serde_json::from_str::<T>(&body) {
            Ok(v) => v,
            Err(e) => {
                println!("=== Provided Json Response ===");
                pretty(&body);
                println!("==============================");
                return Err(e.into());
            }
        };
        Ok(Self(value))
    }
}

#[async_trait]
pub trait IntoBody<R> 
where
    R: FromResponse
{
    async fn into_body(self) -> Result<R>;
}

#[async_trait]
impl<R> IntoBody<R> for Response
where
    R: FromResponse
{
    async fn into_body(self) -> Result<R> {
        R::from_response(self).await
    } 
}
