use std::io;
use tokio_process::{Child, ChildStdin, ChildStdout, Command};

use std::process::Stdio;

use crate::cmd::{Cmd, IOTCM};

pub const INTERACTION_COMMAND: &str = "--interaction-json";

pub struct ProcessStdio {
    pub process: Child,
    pub stdin: ChildStdin,
    pub stdout: ChildStdout,
}

pub fn start_agda(agda_program: &str) -> io::Result<ProcessStdio> {
    let mut process = Command::new(agda_program)
        .arg(INTERACTION_COMMAND)
        .stdout(Stdio::piped())
        .stdin(Stdio::piped())
        .spawn()?; // cannot spawn
    let stdin = process.stdin().take().expect("failed to pipe stdin");
    let stdout = process.stdout().take().expect("failed to pipe stdout");
    Ok(ProcessStdio {
        process,
        stdin,
        stdout,
    })
}

/// Common command: load file in Agda.
pub fn load_file(path: String) -> IOTCM {
    let command = Cmd::Load {
        flags: vec![],
        path: path.clone(),
    };
    IOTCM::new(Default::default(), path, Default::default(), command)
}
