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
    let mut highlighting_info = Vec::with_capacity(5);
    let interaction_points = loop {
        match agda.response().await? {
            Resp::InteractionPoints { interaction_points } => {
                break interaction_points;
            }
            Resp::HighlightingInfo {
                info: Some(mut info),
                filepath: None,
                direct: true,
            } => highlighting_info.append(&mut info.payload),
            Resp::HighlightingInfo {
                info: None,
                filepath: Some(_),
                direct: false,
            } => unimplemented!(),
            Resp::HighlightingInfo { .. } => unreachable!(),
            _ => {}
        }
    };
    if interaction_points.is_empty() {
        println!("Note: no interaction points found.");
    }
    for interaction_point in interaction_points {
        unimplemented!()
    }
    Ok(())
}