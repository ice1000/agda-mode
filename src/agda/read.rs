use std::io;

use serde::Deserialize;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::ChildStdout;

use crate::agda::ReplState;
use crate::debug::debug_response;
use crate::resp::Resp;

/// Deserialize from Agda's command line output.
pub fn deserialize_agda<'a, T: Deserialize<'a>>(buf: &'a str) -> serde_json::Result<T> {
    let buf = buf.trim_start_matches("JSON>").trim();
    serde_json::from_str(buf)
}

#[derive(Debug)]
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
        debug_response(format!("[RES]: {}\n", self.buf));
        let resp = deserialize_agda(&self.buf)?;
        self.buf.clear();
        Ok(resp)
    }
}

impl ReplState {
    /// Await the next Agda response.
    pub async fn response(&mut self) -> io::Result<Resp> {
        self.agda.response().await
    }
}
