use std::io::{self, Write};

use agda_mode::agda::ReplState;
use agda_mode::base::InteractionPoint;
use agda_mode::cmd::{Cmd, GoalInput};
use agda_mode::resp::{DisplayInfo, GoalInfo};

use crate::file_io::{Repl, Monad};
use crate::input::UserInput;
use crate::interact::help;

pub async fn line(agda: &mut Repl, line: &str) -> Monad<bool> {
    line_impl(agda, UserInput::from(line)).await
}

async fn line_impl<'a>(agda: &mut Repl, line: UserInput<'a>) -> Monad<bool> {
    use UserInput::*;
    match line {
        Define(function_name) => {
            let f = &mut agda.file;
            f.write(function_name.as_bytes())?;
            f.write(" : ?\n".as_bytes())?;
            f.write(function_name.as_bytes())?;
            f.write(" = ?\n".as_bytes())?;
            f.flush()?;
            reload(agda).await?;
        }
        Reload => reload(agda).await?,
        Help => {
            println!("{}", help(agda.is_plain));
            // TODO: info for commands.
        }
        Unknown => println!("Sorry, I don't understand."),
        Exit => {
            finish(&mut agda.agda).await?;
            return Ok(true);
        }
    }
    Ok(false)
}

pub async fn reload(agda: &mut Repl) -> Monad {
    reload_impl(&mut agda.agda).await
}

pub async fn reload_impl(agda: &mut ReplState) -> Monad {
    agda.reload_file().await?;
    match agda.next_goals().await? {
        Ok(iis) => {
            println!("Goals:");
            if iis.is_empty() {
                println!("No goals.");
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
