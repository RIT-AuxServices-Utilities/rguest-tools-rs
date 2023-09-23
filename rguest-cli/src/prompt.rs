use std::{process::exit, fmt::Display};
use tokio::io::{AsyncWriteExt, BufReader, AsyncBufReadExt};
use librguest::prelude::*;

pub async fn pick<'a, T: Display> (choices: &'a [T], name: &str) -> &'a T {

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

async fn send_prompt(prompt: &str) -> Result<()> {
    let mut stdout = tokio::io::stdout();
    stdout.write_all(prompt.to_owned().as_bytes()).await?;
    stdout.flush().await?;
    Ok(())
}

pub async fn prompt_string(prompt: &str) -> Result<String, Error> {
    send_prompt(prompt).await?;
    let stdin = tokio::io::stdin();
    let line = BufReader::new(stdin).lines().next_line().await?;
    line.ok_or_else(|| {
        // EOF triggered
        println!("unexpected end of file");
        exit(1);
    })
}

async fn prompt_usize(prompt: &str) -> Result<usize, Error> {
    prompt_string(prompt).await
        .and_then(|s| Ok(s.parse()?))
}

pub async fn prompt_secure(prompt: &str) -> Result<String, Error> {
    send_prompt(prompt).await?;
    Ok(rpassword::read_password()?)
}
