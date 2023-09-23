use std::process::exit;
use colored::Colorize;
use librguest::prelude::*;

pub fn fatal(err: Error) -> ! {
    println!("{} {err}", "Fatal:".bright_red().bold());
    exit(1)
}

#[allow(dead_code)]
pub fn error(err: Error) -> () {
    println!("{} {err}", "Error:".bright_red().bold());
}

pub trait Enforce<T> {
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

