use prelude::Tenant;
use reqwest::Client;

pub mod auth;
pub mod common;
pub mod schedule;
pub mod prelude;

mod extract;

pub struct Context {
    pub tenant: Tenant,
    pub context_id: String,
    client: Client
}

impl Context {
    pub fn new(tenant: Tenant, context_id: String, client: Client) -> Self {
        Self {
            tenant,
            context_id,
            client
        }
    }
}
