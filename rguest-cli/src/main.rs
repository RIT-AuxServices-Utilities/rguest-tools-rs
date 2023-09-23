use crate::prompt::{prompt_string, prompt_secure, pick};
use error::Enforce;
use librguest::{prelude::*, Context};

mod prompt;
mod error;

#[tokio::main]
async fn main() {
    test().await.enforce();
}

async fn test() -> Result<()> {

    println!("Login to rguest!");

    let username = prompt_string("Username: ").await?;
    let password = prompt_secure("Password: ").await?;

    let (login, client) = Login::auth(&username, &password).await?;
    let tenant = pick(&login.tenants, "tenants").await.to_owned();
    println!("Using tenant: {tenant}");

    let roles = tenant.get_roles(&client).await?;
    let role = pick(&roles, "roles").await;
    println!("Using role: {role}");

    let context_id: String;

    if role.id == "Default" {
        let contexts = tenant.get_contexts(&client).await?;
        let context = pick(&contexts, "business contexts").await;
        println!("Using business context: {context}");
        context_id = context.id.to_owned();
    } else {
        context_id = role.id.to_owned();
    }

    let context = Context::new(tenant, context_id, client);
   
    let menus = Menu::get_from_context(&context).await?;
    let locations = Location::get_from_context(&context).await?;
    let dayparts = Daypart::get_from_context(&context).await?;

    println!("Loaded context: {}", &context.tenant.name);
    println!("Menu Count: {}", menus.len());
    println!("Location Count: {}", locations.len());
    println!("Daypart Count: {}", dayparts.len());

    Ok(())
}
