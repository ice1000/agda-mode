use std::io;

use tokio::io::BufReader;
use tokio_process::{ChildStdin, ChildStdout};

use agda_mode::agda::{load_file, send_command, AgdaRead};
use agda_mode::cmd::IOTCM;
use agda_mode::resp::Resp;

pub async fn repl(
    mut stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
    file: String,
) -> io::Result<()> {
    send_command(&mut stdin, &load_file(file)).await?;
    let mut agda = AgdaRead::new(stdout);
    println!("{:?}", agda.response().await?);
    println!("{:?}", agda.response().await?);
    println!("{:?}", agda.response().await?);
    println!("{:?}", agda.response().await?);
    Ok(())
}
