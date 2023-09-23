use reqwest::Response;
use serde::{de::DeserializeOwned, Deserialize};
use async_trait::async_trait;
use anyhow::{Result, Error};

#[async_trait(?Send)]
pub trait FromResponse: Sized {
    async fn from_response(req: Response) -> Result<Self>;
}

#[derive(Debug, Deserialize)]
struct JsonMessage {
    message: String
}

#[derive(Debug)]
pub struct Json<T> (pub T);

#[async_trait(?Send)]
impl<T> FromResponse for Json<T> 
where
    T: DeserializeOwned + Sized
{
    async fn from_response(res: Response) -> Result<Self> {
        let status = res.status();
        let body = res.text().await?;

        if !status.is_success() {
            let Ok(value) = serde_json::from_str::<JsonMessage>(&body) else {
                return Err(Error::msg(format!("request failed with status: {}", status.as_u16())))
            };
            return Err(Error::msg(format!("request failed with {}: {}", status.as_u16(), value.message)))
        }
        let value = serde_json::from_str::<T>(&body)?;
        Ok(Self(value))
    }
}

#[async_trait(?Send)]
pub trait IntoBody<R> 
where
    R: FromResponse
{
    async fn into_body(self) -> Result<R>;
}

#[async_trait(?Send)]
impl<R> IntoBody<R> for Response
where
    R: FromResponse
{
    async fn into_body(self) -> Result<R> {
        R::from_response(self).await
    } 
}
