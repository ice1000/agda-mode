use std::io;

use agda_mode::agda::ReplState;
use agda_mode::cmd::{Cmd, GoalInput};
use agda_mode::resp::{DisplayInfo, GoalInfo, Resp};

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
