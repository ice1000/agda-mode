use std::io;

use serde::Deserialize;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio_process::{ChildStdin, ChildStdout};

use agda_mode::agda::load_file;
use agda_mode::cmd::IOTCM;
use agda_mode::resp::Resp;

pub type In = ChildStdin;
pub type Out = BufReader<ChildStdout>;
pub type ReplMonad<T = ()> = io::Result<T>;

pub fn de<'a, T: Deserialize<'a>>(buf: &'a str) -> serde_json::Result<T> {
    serde_json::from_str(buf.trim_start_matches("JSON>"))
}

pub async fn send_command(stdin: &mut In, command: &IOTCM) -> ReplMonad {
    stdin.write(command.to_string().as_bytes()).await?;
    stdin.flush().await
}

pub async fn response(stdout: &mut Out, buf: &mut String) -> ReplMonad<Resp> {
    stdout.read_line(buf).await?;
    Ok(de(&buf)?)
}

pub async fn repl(mut stdin: In, mut stdout: Out, file: String) -> ReplMonad {
    let mut buf = String::with_capacity(2045);
    send_command(&mut stdin, &load_file(file)).await?;
    let resp = response(&mut stdout, &mut buf).await?;
    println!("{:?}", resp);
    let resp = response(&mut stdout, &mut buf).await?;
    println!("{:?}", resp);
    let resp = response(&mut stdout, &mut buf).await?;
    println!("{:?}", resp);
    let resp = response(&mut stdout, &mut buf).await?;
    println!("{:?}", resp);
    Ok(())
}
