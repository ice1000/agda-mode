use std::io;
use std::process::Stdio;

use serde::Deserialize;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio_process::{Child, ChildStdin, ChildStdout, Command};

use crate::cmd::{Cmd, IOTCM};
use crate::resp::Resp;

pub const INTERACTION_COMMAND: &str = "--interaction-json";
pub const START_FAIL: &str = "Failed to start Agda";

pub struct ProcessStdio(pub Child, pub ChildStdin, pub ChildStdout);

pub fn init_agda_process(agda_program: &str) -> io::Result<ProcessStdio> {
    let mut process = Command::new(agda_program)
        .arg(INTERACTION_COMMAND)
        .stdout(Stdio::piped())
        .stdin(Stdio::piped())
        .spawn()?; // cannot spawn
    let stdin = process.stdin().take().expect("Failed to pipe stdin");
    let stdout = process.stdout().take().expect("Failed to pipe stdout");
    Ok(ProcessStdio(process, stdin, stdout))
}

/// Start the Agda process and return the stdio handles.
///
/// Note that this function may panic.
pub fn start_agda(agda_program: &str) -> (ChildStdin, ChildStdout) {
    let ProcessStdio(process, stdin, stdout) = init_agda_process(agda_program).expect(START_FAIL);
    tokio::spawn(async {
        let status = process.await.expect(START_FAIL);
        println!("Agda exits with status {}.", status);
    });
    (stdin, stdout)
}

/// Deserialize from Agda's command line output.
pub fn deserialize_agda<'a, T: Deserialize<'a>>(buf: &'a str) -> serde_json::Result<T> {
    let buf = buf.trim_start_matches("JSON>").trim();
    serde_json::from_str(buf)
}

/// Send an [`IOTCM`](crate::cmd::IOTCM) command to Agda.
pub async fn send_command(stdin: &mut ChildStdin, command: &IOTCM) -> io::Result<()> {
    stdin.write(command.to_string().as_bytes()).await?;
    stdin.flush().await
}

pub struct AgdaRead {
    buf: String,
    agda: BufReader<ChildStdout>,
}

impl From<BufReader<ChildStdout>> for AgdaRead {
    fn from(agda: BufReader<ChildStdout>) -> Self {
        Self {
            agda,
            buf: String::with_capacity(2048),
        }
    }
}

impl From<ChildStdout> for AgdaRead {
    fn from(o: ChildStdout) -> Self {
        From::from(BufReader::new(o))
    }
}

impl AgdaRead {
    /// Take Agda's response from the next line.
    pub async fn response(&mut self) -> io::Result<Resp> {
        self.agda.read_line(&mut self.buf).await?;
        let resp = deserialize_agda(&self.buf)?;
        self.buf.clear();
        Ok(resp)
    }
}

/// Common command: load file in Agda.
pub fn load_file(path: String) -> IOTCM {
    let command = Cmd::load_simple(path.clone());
    IOTCM::simple( path,  command)
}
