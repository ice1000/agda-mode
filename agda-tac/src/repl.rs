use std::io;

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio_process::{ChildStdin, ChildStdout};

use agda_mode::agda::{load_file, deserialize_agda};
use agda_mode::cmd::IOTCM;
use agda_mode::resp::Resp;

pub type In = ChildStdin;
pub type Out = BufReader<ChildStdout>;
pub type ReplMonad<T = ()> = io::Result<T>;

pub struct AgdaRead {
    buf: String,
    agda: Out,
}

pub async fn send_command(stdin: &mut In, command: &IOTCM) -> ReplMonad {
    stdin.write(command.to_string().as_bytes()).await?;
    stdin.flush().await
}

impl AgdaRead {
    pub  fn new(agda: Out) -> Self {
        Self {
            agda,
            buf: String::with_capacity(2048)
        }
    }

    pub async fn response(&mut self) -> ReplMonad<Resp> {
        self.agda.read_line(&mut self.buf).await?;
        let resp = deserialize_agda(&self.buf)?;
        self.buf.clear();
        Ok(resp)
    }
}

pub async fn repl(mut stdin: In, stdout: Out, file: String) -> ReplMonad {
    send_command(&mut stdin, &load_file(file)).await?;
    let mut agda = AgdaRead::new(stdout);
    println!("{:?}", agda.response().await?);
    println!("{:?}", agda.response().await?);
    println!("{:?}", agda.response().await?);
    println!("{:?}", agda.response().await?);
    Ok(())
}
