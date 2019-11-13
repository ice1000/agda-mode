use std::io;

use tokio::io::BufReader;
use tokio_process::{ChildStdin, ChildStdout};

use agda_mode::agda::{load_file, send_command, AgdaRead};
use agda_mode::cmd::Cmd;
use agda_mode::resp::Resp;

pub async fn repl(
    mut stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
    file: String,
) -> io::Result<()> {
    let mut iotcm = load_file(file);
    send_command(&mut stdin, &iotcm).await?;
    let mut agda = AgdaRead::from(stdout);
    let interaction_points = loop {
        if let Resp::InteractionPoints { interaction_points } = agda.response().await? {
            break interaction_points;
        }
    };
    if interaction_points.is_empty() {
        println!("No interaction points found.");
    }
    for interaction_point in interaction_points {
        unimplemented!()
    }
    Ok(())
}
