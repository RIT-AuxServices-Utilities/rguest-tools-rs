use std::process::exit;

use tokio::io::{AsyncWriteExt, BufReader, AsyncBufReadExt};
use anyhow::Error;

async fn send_prompt(prompt: &str) -> Result<(), Error> {
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

pub async fn prompt_usize(prompt: &str) -> Result<usize, Error> {
    prompt_string(prompt).await
        .and_then(|s| Ok(s.parse()?))
}

pub async fn prompt_secure(prompt: &str) -> Result<String, Error> {
    send_prompt(prompt).await?;
    Ok(rpassword::read_password()?)
}
