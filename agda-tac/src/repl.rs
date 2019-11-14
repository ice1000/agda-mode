use std::io;

use tokio::io::{AsyncWriteExt, BufReader};
use tokio_process::{ChildStdin, ChildStdout};

use agda_mode::agda::{load_file, send_command, start_agda, AgdaRead};
use agda_mode::cmd::{Cmd, GoalInput, IOTCM};
use agda_mode::resp::{DisplayInfo, GoalInfo, Resp};

pub struct ReplState {
    pub stdin: ChildStdin,
    pub agda: AgdaRead,
    pub file: String,
    iotcm: IOTCM,
}

impl ReplState {
    pub async fn start(agda_program: &str, file: String) -> io::Result<Self> {
        let (stdin, out) = start_agda(agda_program);
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

    pub async fn command(&mut self, cmd: Cmd) -> io::Result<()> {
        self.iotcm.command = cmd;
        send_command(&mut self.stdin, &self.iotcm).await
    }

    pub async fn shutdown(&mut self) -> io::Result<()> {
        self.stdin.shutdown().await
    }

    pub async fn response(&mut self) -> io::Result<Resp> {
        self.agda.response().await
    }
}

pub async fn repl(mut agda: ReplState) -> io::Result<()> {
    let iis = loop {
        if let Resp::InteractionPoints { interaction_points } = agda.response().await? {
            break interaction_points;
        }
    };
    if iis.is_empty() {
        println!("Note: no interaction points found.");
    }
    for ii in iis {
        agda.command(Cmd::goal_type(GoalInput::simple(ii))).await?;
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
        println!("?{:?}: {}", ii, ty);
    }
    agda.command(Cmd::Abort).await?;
    agda.shutdown().await
}
