use std::io;
use std::process::Stdio;

use serde::Deserialize;
use tokio::io::AsyncWriteExt;
use tokio::net::process::{Child, ChildStdin, ChildStdout, Command};

use crate::cmd::{Cmd, IOTCM};
use crate::debug::debug_command;

pub use self::read::*;
pub use self::repl::*;

mod read;
mod repl;

pub const INTERACTION_COMMAND: &str = "--interaction-json";
pub const START_FAIL: &str = "Failed to start Agda";

pub struct ProcessStdio(pub Child, pub JustStdio);

pub struct JustStdio(pub ChildStdin, pub ChildStdout);

pub fn init_agda_process(agda_program: &str) -> io::Result<ProcessStdio> {
    let mut process = Command::new(agda_program)
        .arg(INTERACTION_COMMAND)
        .stdout(Stdio::piped())
        .stdin(Stdio::piped())
        .spawn()?; // cannot spawn
    let stdin = process.stdin().take().expect("Failed to pipe stdin");
    let stdout = process.stdout().take().expect("Failed to pipe stdout");
    // The above two should not panic, because both stdio are piped
    Ok(ProcessStdio(process, JustStdio(stdin, stdout)))
}

/// Start the Agda process and return the stdio handles.
///
/// Note that this function may panic.
pub fn start_agda(agda_program: &str) -> JustStdio {
    let ProcessStdio(process, stdio) = init_agda_process(agda_program).expect(START_FAIL);
    tokio::spawn(async {
        let status = process.await.expect(START_FAIL);
        println!("Agda exits with status {}.", status);
    });
    stdio
}

/// Deserialize from Agda's command line output.
pub fn deserialize_agda<'a, T: Deserialize<'a>>(buf: &'a str) -> serde_json::Result<T> {
    let buf = buf.trim_start_matches("JSON>").trim();
    serde_json::from_str(buf)
}

/// Send an [`IOTCM`](crate::cmd::IOTCM) command to Agda.
pub async fn send_command(stdin: &mut ChildStdin, command: &IOTCM) -> io::Result<()> {
    let string = command.to_string();
    unsafe { debug_command(format!("[CMD]: {}", string)) };
    stdin.write(string.as_bytes()).await?;
    stdin.flush().await
}

/// Common command: load file in Agda.
pub fn load_file(path: String) -> IOTCM {
    let command = Cmd::load_simple(path.clone());
    IOTCM::simple(path, command)
}
