use crate::extract::{Json, IntoBody};

use super::tenant::Tenant;
use reqwest::{header::{HeaderMap, COOKIE, HeaderValue}, Client};
use serde::Deserialize;
use anyhow::Result;
use serde_json::json;

#[derive(Deserialize)]
pub struct Login {
    token: String,
    pub tenants: Vec<Tenant>,
}

impl Login {

    async fn recreate_client(token: &str) -> Result<Client> {
        let mut headers = HeaderMap::new();
        headers.insert(
            COOKIE,
            HeaderValue::from_str(&format!("X-Token={}", token))?
        );
        headers.insert(
            "x-token",
            HeaderValue::from_str(token)?
        );

        Ok(Client::builder()
            .gzip(true)
            .cookie_store(true)
            .default_headers(headers)
            .build()?)
    }

    pub async fn auth(username: &str, password: &str) -> Result<(Self, Client)> {

        let req = Client::new()
            .post("https://buy.rguest.com/auth-service/auth/users/login")
            .header("content-type", "application/json")
            .body(json!({
                "username": username,
                "password": password
            }).to_string());

        let res = req.send().await?;

        let Json(login): Json<Self> = res.into_body().await?;

        let client = Self::recreate_client(&login.token).await?;
        
        Ok((login, client))
    }

}
