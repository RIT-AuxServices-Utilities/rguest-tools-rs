mod api;
mod extract;
mod prompt;

use std::process::exit;

use colored::Colorize;
use anyhow::Error;

use crate::api::auth::context::Context;

pub fn fatal(err: Error) -> ! {
    println!("{} {err}", "Fatal:".bright_red().bold());
    exit(1)
}

pub fn error(err: Error) -> () {
    println!("{} {err}", "Error:".bright_red().bold());
}

trait Enforce<T> {
    fn enforce(self) -> T;
}

impl<T> Enforce<T> for Result<T, Error> {
    fn enforce(self) -> T {
        match self {
            Ok(suc) => suc,
            Err(err) => fatal(err),
        }
    }
}

#[tokio::main]
async fn main() {

    let ctx = Context::new().await.enforce();
    println!("{ctx:?}");

}
