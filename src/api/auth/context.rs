use reqwest::{header::{HeaderMap, COOKIE, HeaderValue}, Client};
use serde::Deserialize;
use serde_json::json;
use anyhow::Result;
use crate::{api::{API_CLIENT, auth::pick}, prompt::{prompt_string, prompt_secure}, extract::{IntoBody, Json}};
use super::tenant::Tenant;

#[derive(Deserialize, Debug)]
struct LoginRquest {
    token: String,
    tenants: Vec<Tenant>,
}

#[derive(Debug)]
pub struct Context {
    pub tenant: Tenant,
    pub context_id: String,
}

impl Context {

    async fn auth() -> Result<Json<LoginRquest>> {
            println!("Please login to rGuest:");
    
            let username = prompt_string("Username: ").await?;
            let password = prompt_secure("Password: ").await?;

         let req = API_CLIENT.lock().await
            .post("https://buy.rguest.com/auth-service/auth/users/login")
            .header("content-type", "application/json")
            .body(json!({
                "username": username,
                "password": password
            }).to_string());

        let res = req.send().await?;

        res.into_body().await
    }

    async fn update_client(token: &str) -> Result<()> {

        let mut headers = HeaderMap::new();
        headers.insert(
            COOKIE,
            HeaderValue::from_str(&format!("X-Token={}", token))?
        );
        headers.insert(
            "x-token",
            HeaderValue::from_str(token)?
        );

        *(API_CLIENT.lock().await) = Client::builder()
            .gzip(true)
            .cookie_store(true)
            .default_headers(headers)
            .build()?;

        Ok(())
    }

    pub async fn new() -> Result<Self> {

        let Json(auth): Json<LoginRquest> = Self::auth().await?;
        Self::update_client(&auth.token).await?;
        
        let tenant = pick(&auth.tenants, "tennants").await.clone();
        
        println!("Using tenant: {tenant}");

        let context_id = tenant.get_context_id().await?;

        Ok(Self {
            tenant,
            context_id
        })
    }

}
