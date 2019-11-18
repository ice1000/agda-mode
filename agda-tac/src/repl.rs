use agda_mode::agda::{preprint_agda_result, ReplState};
use agda_mode::base::ComputeMode;
use agda_mode::cmd::{Cmd, GoalInput};
use agda_mode::pos::{InteractionId};
use agda_mode::resp::GoalInfo;

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
            agda.append_line(&format!("{} : ?", function_name))?;
            agda.append_line(&format!("{} = ?", function_name))?;
            reload(agda).await?;
        }
        RawLine(code) => {
            agda.append_line(code)?;
            reload(agda).await?;
        }
        Give(i, new) => {
            let command = Cmd::give(GoalInput::no_range(i, new.to_owned()));
            agda.agda.command(command).await?;
            // TODO: write to buffer
            preprint_agda_result(agda.agda.next_give_action().await?, |gs| unimplemented!());
        }
        Infer(i, new) => {
            let command = Cmd::infer(GoalInput::no_range(i, new.to_owned()));
            agda.agda.command(command).await?;
            preprint_agda_result(agda.agda.next_goal_specific().await?, |gs| {
                match gs.goal_info {
                    GoalInfo::InferredType { expr } => println!("{} : {}", new, expr),
                    _ => unreachable!(),
                }
            });
        }
        Simplify(i, new) => norm(agda, i, new, ComputeMode::DefaultCompute).await?,
        Normalize(i, new) => norm(agda, i, new, ComputeMode::UseShowInstance).await?,
        Type(i) => {
            let command = Cmd::goal_type(GoalInput::simple(i));
            agda.agda.command(command).await?;
            preprint_agda_result(agda.agda.next_goal_specific().await?, |gs| {
                match gs.goal_info {
                    GoalInfo::CurrentGoal { the_type, .. } => println!("{}", the_type),
                    _ => unreachable!(),
                }
            });
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

async fn norm(agda: &mut Repl, i: InteractionId, new: &str, mode: ComputeMode) -> Monad {
    let command = Cmd::Compute {
        compute_mode: mode,
        input: GoalInput::no_range(i, new.to_owned()),
    };
    agda.agda.command(command).await?;
    preprint_agda_result(agda.agda.next_goal_specific().await?, |gs| {
        match gs.goal_info {
            GoalInfo::NormalForm { expr, .. } => println!("{} --> {}", new, expr),
            _ => unreachable!(),
        }
    });
    Ok(())
}

pub async fn reload(agda: &mut Repl) -> Monad {
    let da = &mut agda.agda;
    da.reload_file().await?;
    poll_goals(da).await
}

pub async fn poll_goals(agda: &mut ReplState) -> Monad {
    preprint_agda_result(agda.next_all_goals_warnings().await?, |agw| {
        if agw.visible_goals.is_empty() {
            println!("No goals.");
        } else {
            println!("Goals:");
        }
        for goal in agw.visible_goals {
            // I believe `goal` will always be `OfType`.
            match goal.try_as_of_type() {
                Ok(ok) => println!("?{} : {}", ok.constraint_obj, ok.of_type),
                Err(bad) => eprintln!("[WARN]: unexpected goal: {:?}", bad),
            }
        }
        if !agw.invisible_goals.is_empty() {
            println!("Unsolved metas:");
        }
        for meta in agw.invisible_goals {
            println!("{}", meta);
        }
    });
    Ok(())
}

async fn finish(agda: &mut ReplState) -> Monad {
    agda.command(Cmd::Abort).await?;
    agda.shutdown().await
}
