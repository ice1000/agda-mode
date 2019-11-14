use std::io;

use tokio::io::BufReader;
use tokio_process::{ChildStdin, ChildStdout};

use agda_mode::agda::{load_file, send_command, AgdaRead};
use agda_mode::cmd::{Cmd, GoalInput};
use agda_mode::resp::{DisplayInfo, GoalInfo, Resp};

pub async fn repl(
    mut stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
    file: String,
) -> io::Result<()> {
    let mut iotcm = load_file(file);
    send_command(&mut stdin, &iotcm).await?;
    let mut agda = AgdaRead::from(stdout);
    let iis = loop {
        if let Resp::InteractionPoints { interaction_points } = agda.response().await? {
            break interaction_points;
        }
    };
    if iis.is_empty() {
        println!("Note: no interaction points found.");
    }
    for ii in iis {
        iotcm.command = Cmd::goal_type(GoalInput::simple(ii));
        send_command(&mut stdin, &iotcm).await?;
        let ty = loop {
            if let Resp::DisplayInfo {
                info:
                    Some(DisplayInfo::GoalSpecific {
                        goal_info: GoalInfo::CurrentGoal { the_type, .. },
                        ..
                    }),
            } = agda.response().await?
            {
                break the_type;
            }
        };
        println!("Point {:?}:", ii);
        println!("  Expected: {}", ty);
    }
    iotcm.command = Cmd::Abort;
    send_command(&mut stdin, &iotcm).await
}
