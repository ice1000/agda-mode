use either::Either;

use agda_mode::agda::{preprint_agda_result, ReplState};
use agda_mode::base::ComputeMode;
use agda_mode::cmd::{Cmd, GoalInput};
use agda_mode::debug::{toggle_debug_command, toggle_debug_response};
use agda_mode::pos::InteractionId;
use agda_mode::resp::GoalInfo;

use crate::file_io::{Monad, Repl};
use crate::input::{UserInput, HELP};
use crate::interact::help;

pub use self::goal::*;
pub use self::goal_list::*;

mod goal;
mod goal_list;

pub async fn line(agda: &mut Repl, line: &str) -> Monad<bool> {
    line_impl(agda, UserInput::from(line)).await
}

async fn line_impl<'a>(agda: &mut Repl, line: UserInput<'a>) -> Monad<bool> {
    use UserInput::*;
    match line {
        Define(function_name) => {
            agda.append(&format!("{} : ?\n", function_name))?;
            agda.append(&format!("{} = ?\n", function_name))?;
            reload(agda).await?;
        }
        PushLine(code) => {
            agda.append(code)?;
            agda.append("\n")?;
            reload(agda).await?;
        }
        PopLine => {
            agda.remove_last_line()?;
            reload(agda).await?;
        }
        ShowLine(i) => {
            let line_max = agda.line_count();
            if i >= line_max {
                eprintln!("There are only {} lines in total.", line_max);
            } else {
                print!("{}", agda.line_in_buffer(i))
            }
        }
        Give(i, new) => give(agda, i, new),
        Infer(i, new) => infer(agda, i, new),
        Simplify(i, new) => norm(agda, i, new, ComputeMode::DefaultCompute).await?,
        Normalize(i, new) => norm(agda, i, new, ComputeMode::UseShowInstance).await?,
        Type(i) => {
            let command = Cmd::goal_type(GoalInput::simple(i));
            agda.agda.command(command).await?;
            if let Some(gs) = preprint_agda_result(agda.agda.next_goal_specific().await?) {
                match gs.goal_info {
                    GoalInfo::CurrentGoal { the_type, .. } => println!("{}", the_type),
                    _ => unreachable!(),
                }
            }
        }
        Reload => {
            reload(agda).await?;
        }
        ReadToEnd => loop {
            agda.agda.response().await?;
        },
        SearchModule(s) => {
            let command = Cmd::search_module(s.to_owned());
            agda.agda.command(command).await?;
            let e: String = agda.agda.next_error().await?.into();
            eprintln!("Error:");
            eprintln!("{}", e);
        }
        ListGoals => {
            let ips = agda.agda.interaction_points();
            if ips.is_empty() {
                println!("No goals, you're all set.");
            }
            for interaction_point in ips {
                // This shouldn't fail
                let range = &interaction_point.range;
                debug_assert_eq!(range.len(), 1);
                let interval = &range[0];
                println!("?{} at line {}", interaction_point.id, interval.start.line)
            }
        }
        Help => {
            println!("{}", help(agda.is_plain));
            for line in HELP {
                println!("{}", line);
            }
        }
        ToggleDebugCommand => unsafe { toggle_debug_command() },
        ToggleDebugResponse => unsafe { toggle_debug_response() },
        Unknown(Some(err)) => println!("Wait, {}", err),
        Unknown(None) => println!("Sorry, I don't understand."),
        Exit => {
            finish(&mut agda.agda).await?;
            return Ok(true);
        }
    }
    Ok(false)
}

async fn finish(agda: &mut ReplState) -> Monad {
    agda.command(Cmd::Abort).await?;
    agda.shutdown().await
}
