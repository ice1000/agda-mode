use std::io;
use std::process::Stdio;

use serde::Deserialize;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::process::{Child, ChildStdin, ChildStdout, Command};

use crate::base::InteractionPoint;
use crate::cmd::{Cmd, IOTCM};
use crate::debug::{debug_command, debug_response};
use crate::resp::{AllGoalsWarnings, DisplayInfo, GoalSpecific, Resp};

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
        unsafe { debug_response(format!("[RES]: {}\n", self.buf)) };
        let resp = deserialize_agda(&self.buf)?;
        self.buf.clear();
        Ok(resp)
    }
}

/// Common command: load file in Agda.
pub fn load_file(path: String) -> IOTCM {
    let command = Cmd::load_simple(path.clone());
    IOTCM::simple(path, command)
}

/// Simple REPL state wrapper.
pub struct ReplState {
    pub stdin: ChildStdin,
    pub agda: AgdaRead,
    pub file: String,
    iotcm: IOTCM,
}

/// An Agda response that is either something good or some error.
pub type AgdaResult<T> = Result<T, String>;
/// Return type of `next_*` functions.
pub type NextResult<T> = io::Result<AgdaResult<T>>;

pub fn preprint_agda_result<T>(t: AgdaResult<T>, f: impl FnOnce(T)) {
    match t {
        Ok(o) => f(o),
        Err(e) => {
            eprintln!("Errors:");
            eprintln!("{}", e);
        }
    }
}

impl ReplState {
    pub async fn start(agda_program: &str, file: String) -> io::Result<Self> {
        let JustStdio(stdin, out) = start_agda(agda_program);
        Self::from_io(stdin, BufReader::new(out), file).await
    }

    pub async fn from_io(
        mut stdin: ChildStdin,
        stdout: BufReader<ChildStdout>,
        file: String,
    ) -> io::Result<Self> {
        let iotcm = load_file(file.clone());
        send_command(&mut stdin, &iotcm).await?;
        let agda = AgdaRead::from(stdout);
        Ok(Self {
            file,
            iotcm,
            stdin,
            agda,
        })
    }

    pub async fn reload_file(&mut self) -> io::Result<()> {
        self.command(Cmd::load_simple(self.file.clone())).await
    }

    pub async fn command(&mut self, cmd: Cmd) -> io::Result<()> {
        self.iotcm.command = cmd;
        send_command(&mut self.stdin, &self.iotcm).await
    }

    pub async fn shutdown(&mut self) -> io::Result<()> {
        self.stdin.shutdown().await
    }

    /// Await the next Agda response.
    pub async fn response(&mut self) -> io::Result<Resp> {
        self.agda.response().await
    }

    /// Skip information until the next display info.
    pub async fn next_display_info(&mut self) -> io::Result<DisplayInfo> {
        loop {
            match self.response().await? {
                Resp::DisplayInfo { info: Some(info) } => break Ok(info),
                _ => {}
            }
        }
    }

    /// Skip information until the next interaction point (goal) list.
    pub async fn next_goals(&mut self) -> NextResult<Vec<InteractionPoint>> {
        use crate::resp::DisplayInfo::Error as DisError;
        use Resp::*;
        loop {
            match self.response().await? {
                InteractionPoints { interaction_points } => break Ok(Ok(interaction_points)),
                DisplayInfo {
                    info: Some(DisError(e)),
                } => break Ok(e.into()),
                _ => {}
            }
        }
    }

    /// Skip information until the next goal specific information.
    pub async fn next_goal_specific(&mut self) -> NextResult<GoalSpecific> {
        use crate::resp::DisplayInfo::Error as DisError;
        use crate::resp::DisplayInfo::GoalSpecific as DisGS;
        loop {
            match self.next_display_info().await? {
                DisError(e) => break Ok(e.into()),
                DisGS(payload) => break Ok(Ok(payload)),
                _ => {}
            }
        }
    }

    /// Skip information until the next interaction point (goal) list.
    pub async fn next_all_goals_warnings(&mut self) -> NextResult<AllGoalsWarnings> {
        use crate::resp::DisplayInfo::AllGoalsWarnings as DisAGW;
        use crate::resp::DisplayInfo::Error as DisError;
        loop {
            match self.next_display_info().await? {
                DisError(e) => break Ok(e.into()),
                DisAGW(payload) => break Ok(Ok(payload)),
                _ => {}
            }
        }
    }
}
