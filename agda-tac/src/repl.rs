use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio_process::{ChildStdin, ChildStdout};
use serde::Deserialize;

use agda_mode::agda::load_file;
use agda_mode::resp::Resp;
use std::io;

pub type In = ChildStdin;
pub type Out = BufReader<ChildStdout>;
pub type ReplMonad<T = ()> = io::Result<T>;

pub fn de<'a, T: Deserialize<'a>>(buf: &'a str) -> ReplMonad<T> {
    let de = serde_json::from_str(buf.trim_start_matches("JSON>"))?;
    Ok(de)
}

pub async fn repl(mut stdin: In, mut stdout: Out, file: String) -> ReplMonad {
    let load_file = load_file(file);
    let mut buf = String::with_capacity(2045);
    stdin.write(load_file.to_string().as_bytes()).await?;
    stdin.flush().await?;
    stdout.read_line(&mut buf).await?;
    let resp: Resp = de(&buf)?;
    println!("{:?}", resp);
    Ok(())
}
