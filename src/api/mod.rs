use reqwest::Client;
use lazy_static::lazy_static;
use tokio::sync::Mutex;

pub mod auth;
pub mod common;

pub mod prelude {
    pub use super::auth::tenant::Tenant as Tenant;
    pub use super::auth::role::Role as Role;
    pub use super::auth::context::Context as Context;
    pub use super::common::menu::Menu as Menu;
    pub use super::common::daypart::Daypart as Daypart;
    pub use super::common::location::Location as Location;
}

lazy_static! {
    pub static ref API_CLIENT: Mutex<Client> = Mutex::new(Client::builder()
        .gzip(true)
        .cookie_store(true)
        .build()
        .unwrap());
}

