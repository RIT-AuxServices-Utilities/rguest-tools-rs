use std::{string::FromUtf8Error, io::Read};
use reqwest::Response;
use serde::{de::DeserializeOwned, Deserialize};

#[derive(Deserialize, Debug)]
pub struct ApiError {
    pub status: u16,
    pub message: String
}

#[derive(Debug)]
pub enum Error {
    FailedToReadBytes(reqwest::Error),
    FailedToParseUTF8(FromUtf8Error),
    FailedToParseBody(Box<str>),
    ApiError(ApiError)
}

async fn parse_body(res: Response) -> Result<String, Error> {

    let bytes = res.bytes().await
        .map_err(|e| Error::FailedToReadBytes(e))?;

    let body = String::from_utf8(bytes.bytes().flatten().collect())
        .map_err(|e| Error::FailedToParseUTF8(e))?;

    Ok(body)
}

pub struct Json<T> (pub T);

impl<T> Json<T> 
where
    T: DeserializeOwned
{
    pub async fn from(res: Response) -> Result<Self, Error> {
        let status = res.status();

        let body = parse_body(res).await?;

        if status.as_u16() > 201 {
            let value = serde_json::from_str::<ApiError>(&body)
                .map_err(|e| Error::FailedToParseBody(format!("{}", e).into()))?;
            Err(Error::ApiError(value))
        } else {
            let value = serde_json::from_str::<T>(&body)
                .map_err(|e| Error::FailedToParseBody(format!("{}", e).into()))?; 
            Ok(Self(value))
        }
    }
}
