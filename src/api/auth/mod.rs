use std::fmt::Display;

use crate::prompt::prompt_usize;
pub mod context;
pub mod role;
pub mod tenant;

async fn pick<'a, T: Display> (choices: &'a [T], name: &str) -> &'a T {

    if choices.len() < 2 {
        if let Some(choice) = choices.iter().nth(0) {
            return choice
        }
    }

    println!("Multiple {name} were found, please pick one.");

    for (index, choice) in choices.iter().enumerate() {
        println!("{index}. {choice}");
    }

    loop {
        let Ok(num) = prompt_usize("> ").await else { continue };
        let Some(choice) = choices.iter().nth(num) else { continue };
        return choice
    }

}
