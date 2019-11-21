use crate::agda::deserialize_agda;
use crate::debug::debug_response;
use crate::resp::Resp;
use std::io;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::process::ChildStdout;

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
