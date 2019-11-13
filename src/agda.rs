use std::io;
use std::process::Stdio;

use serde::Deserialize;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio_process::{Child, ChildStdin, ChildStdout, Command};

use crate::cmd::{Cmd, IOTCM};
use crate::resp::Resp;

pub const INTERACTION_COMMAND: &str = "--interaction-json";

pub struct ProcessStdio(pub Child, pub ChildStdin, pub ChildStdout);

pub fn start_agda(agda_program: &str) -> io::Result<ProcessStdio> {
    let mut process = Command::new(agda_program)
        .arg(INTERACTION_COMMAND)
        .stdout(Stdio::piped())
        .stdin(Stdio::piped())
        .spawn()?; // cannot spawn
    let stdin = process.stdin().take().expect("Failed to pipe stdin");
    let stdout = process.stdout().take().expect("Failed to pipe stdout");
    Ok(ProcessStdio(process, stdin, stdout))
}

pub fn deserialize_agda<'a, T: Deserialize<'a>>(buf: &'a str) -> serde_json::Result<T> {
    serde_json::from_str(buf.trim_start_matches("JSON>").trim())
}

pub async fn send_command(stdin: &mut ChildStdin, command: &IOTCM) -> io::Result<()> {
    stdin.write(command.to_string().as_bytes()).await?;
    stdin.flush().await
}

pub struct AgdaRead {
    buf: String,
    agda: BufReader<ChildStdout>,
}

impl AgdaRead {
    pub  fn new(agda: BufReader<ChildStdout>) -> Self {
        Self {
            agda,
            buf: String::with_capacity(2048)
        }
    }

    pub async fn response(&mut self) -> io::Result<Resp> {
        self.agda.read_line(&mut self.buf).await?;
        let resp = deserialize_agda(&self.buf)?;
        self.buf.clear();
        Ok(resp)
    }
}

/// Common command: load file in Agda.
pub fn load_file(path: String) -> IOTCM {
    let command = Cmd::Load {
        flags: vec![],
        path: path.clone(),
    };
    IOTCM::new(Default::default(), path, Default::default(), command)
}
