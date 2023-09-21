use std::io::{self, Write};
use reqwest::Client;
use rpassword::prompt_password;
use serde::Deserialize;
use serde_json::json;

use crate::extract::{Json, self};

#[derive(Debug)]
pub enum Error {
    FailReadUsername(io::Error),
    FailReadPassword(io::Error),
    FailSendRequest(reqwest::Error),
    FailParseResponse(extract::Error)
}

#[derive(Deserialize, Debug)]
pub struct Token {
    pub token: String
}

impl Token {
    
    fn creds() -> Result<(String, String), Error> {
        let mut username = String::new();
        let stdin = io::stdin();
        print!("Username: ");
        let _ = io::stdout().flush();
        stdin.read_line(&mut username)
            .map_err(|e| Error::FailReadUsername(e))?;
        username.truncate(username.len() - 1);

        let password = prompt_password("Password: ")
            .map_err(|e| Error::FailReadPassword(e))?;

        Ok((username, password))
    }

    pub async fn new() -> Result<Self, Error> {

        println!("Please login to rGuest:");

        let (username, password) = Self::creds()?;

        let client = Client::new()
            .post("https://buy.rguest.com/auth-service/auth/users/login")
            .header("content-type", "application/json")
            .body(json!({
                "username": username,
                "password": password
            }).to_string());

        let res = client.send().await
            .map_err(|e| Error::FailSendRequest(e))?;

        let token: Json<Self> = Json::from(res).await
            .map_err(|e| Error::FailParseResponse(e))?;
        
        Ok(token.0)
    }

}
