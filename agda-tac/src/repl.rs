use agda_mode::agda::ReplState;
use agda_mode::base::InteractionPoint;
use agda_mode::cmd::{Cmd, GoalInput};
use agda_mode::resp::{DisplayInfo, GoalInfo};

use crate::file_io::{Monad, Repl};
use crate::input::UserInput;
use crate::interact::help;

pub async fn line(agda: &mut Repl, line: &str) -> Monad<bool> {
    line_impl(agda, UserInput::from(line)).await
}

async fn line_impl<'a>(agda: &mut Repl, line: UserInput<'a>) -> Monad<bool> {
    use UserInput::*;
    match line {
        Define(function_name) => {
            agda.append_line(format!("{} : ?", function_name))?;
            agda.append_line(format!("{} = ?", function_name))?;
            reload(agda).await?;
        }
        Give(i, new) => {
            let command = Cmd::give(GoalInput::no_range(i, new.to_owned()));
            agda.agda.command(command).await?;
            // TODO: write to buffer
            // TODO: check for error message & successful give result
        }
        Reload => reload(agda).await?,
        Help => {
            println!("{}", help(agda.is_plain));
            // TODO: info for commands.
        }
        Unknown(Some(err)) => println!("Wait, {}", err),
        Unknown(None) => println!("Sorry, I don't understand."),
        Exit => {
            finish(&mut agda.agda).await?;
            return Ok(true);
        }
    }
    Ok(false)
}

pub async fn reload(agda: &mut Repl) -> Monad {
    let da = &mut agda.agda;
    da.reload_file().await?;
    poll_goals(da).await
}

pub async fn poll_goals(agda: &mut ReplState) -> Monad {
    match agda.next_goals().await? {
        Ok(iis) => {
            if iis.is_empty() {
                println!("No goals.");
            } else {
                println!("Goals:");
            }
            list_goals(agda, &iis).await?;
        }
        Err(err_msg) => {
            eprintln!("Errors:");
            eprintln!("{}", err_msg);
        }
    }
    Ok(())
}

async fn finish(agda: &mut ReplState) -> Monad {
    agda.command(Cmd::Abort).await?;
    agda.shutdown().await
}

async fn list_goals(agda: &mut ReplState, iis: &[InteractionPoint]) -> Monad {
    for &ii in iis {
        agda.command(Cmd::goal_type(GoalInput::simple(ii))).await?;
        let ty = loop {
            if let DisplayInfo::GoalSpecific {
                goal_info: GoalInfo::CurrentGoal { the_type, .. },
                ..
            } = agda.next_display_info().await?
            {
                break the_type;
            }
        };
        println!("?{:?}: {}", ii, ty);
    }
    Ok(())
}
