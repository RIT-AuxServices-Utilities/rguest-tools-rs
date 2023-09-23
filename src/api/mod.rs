use reqwest::Client;
use lazy_static::lazy_static;
use tokio::sync::Mutex;

pub mod auth;

lazy_static! {
    pub static ref API_CLIENT: Mutex<Client> = Mutex::new(Client::builder()
        .gzip(true)
        .cookie_store(true)
        .build()
        .unwrap());
}

