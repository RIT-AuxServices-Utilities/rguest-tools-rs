mod auth;
mod extract;

use crate::auth::Token;

#[tokio::main]
async fn main() {

    let token = Token::new().await.unwrap();
    println!("{:?}", token);
}
