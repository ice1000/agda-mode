use std::io;
use std::process::Stdio;

use tokio::io::{AsyncWriteExt, BufReader};
use tokio::net::process::{Child, ChildStdin, ChildStdout, Command};

use crate::cmd::{Cmd, IOTCM};
use crate::debug::debug_command;

pub use self::read::*;
pub use self::repl::*;

/// Agda message reading.
mod read;
/// Repl state wrapper.
mod repl;
/// Verify whether Agda is working.
pub mod verify;

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

impl ReplState {
    pub async fn start(agda_program: &str, file: String) -> io::Result<Self> {
        let JustStdio(stdin, out) = start_agda(agda_program);
        Self::from_io(stdin, BufReader::new(out), file).await
    }

    pub async fn from_io(
        stdin: ChildStdin,
        stdout: BufReader<ChildStdout>,
        file: String,
    ) -> io::Result<Self> {
        let iotcm = load_file(file.clone());
        Ok(Self {
            file,
            iotcm,
            stdin,
            interaction_points: vec![],
            agda: AgdaRead::from(stdout),
        })
    }
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
