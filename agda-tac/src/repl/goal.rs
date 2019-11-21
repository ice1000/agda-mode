use agda_mode::agda::preprint_agda_result;
use agda_mode::base::ComputeMode;
use agda_mode::cmd::{Cmd, GoalInput};
use agda_mode::pos::InteractionId;
use agda_mode::resp::{Context, GoalInfo, MakeCase, MakeCaseVariant};

use crate::file_io::{Monad, Repl};
use either::Either;

pub async fn norm(agda: &mut Repl, i: InteractionId, new: &str, mode: ComputeMode) -> Monad {
    let command = Cmd::Compute {
        compute_mode: mode,
        input: GoalInput::no_range(i, new.to_owned()),
    };
    agda.agda.command(command).await?;
    if let Some(gs) = preprint_agda_result(agda.agda.next_goal_specific().await?) {
        match gs.goal_info {
            GoalInfo::NormalForm { expr, .. } => println!("{} --> {}", new, expr),
            _ => unreachable!(),
        }
    }
    Ok(())
}

pub async fn give(agda: &mut Repl, i: InteractionId, new: &str) -> Monad {
    let command = Cmd::give(GoalInput::no_range(i, new.to_owned()));
    agda.agda.command(command).await?;
    if let Some(gs) = preprint_agda_result(agda.agda.next_give_action().await?) {
        match gs.give_result.into_either() {
            Either::Left(s) => agda.fill_goal_buffer(gs.interaction_point, &s),
            // Don't know yet what to do
            Either::Right(_b) => unimplemented!(),
        }
        agda.sync_buffer()?;
        // Poll the goals' information
        agda.agda.next_goals().await?;
    }
    Ok(())
}

pub async fn infer(agda: &mut Repl, i: InteractionId, new: &str) -> Monad {
    let command = Cmd::infer(GoalInput::no_range(i, new.to_owned()));
    agda.agda.command(command).await?;
    if let Some(gs) = preprint_agda_result(agda.agda.next_goal_specific().await?) {
        match gs.goal_info {
            GoalInfo::InferredType { expr } => println!("{} : {}", new, expr),
            _ => unreachable!(),
        }
    }
    Ok(())
}

pub async fn split(agda: &mut Repl, i: InteractionId, pat: &str) -> Monad {
    let command = Cmd::split(GoalInput::no_range(i, pat.to_owned()));
    agda.agda.command(command).await?;
    if let Some(mk) = preprint_agda_result(agda.agda.next_make_case().await?) {
        let mk: MakeCase = mk;
        match mk.variant {
            MakeCaseVariant::Function => {
                let start = mk.interaction_point.the_interval().start;
                let line = start.line;
                // Double-check the position.
                // Note Agda uses 1-indexed line numbers.
                debug_assert_eq!(agda.line_of_offset(start.pos) + 1, line);
                agda.remove_line_buffer(line);
                for clause in mk.clauses.into_iter().rev() {
                    agda.insert_line_buffer(line, &clause);
                }
            }
            MakeCaseVariant::ExtendedLambda => unimplemented!(),
        }
        agda.sync_buffer()?;
    }
    Ok(())
}

pub async fn ctx(agda: &mut Repl, i: InteractionId) -> Monad {
    let command = Cmd::context(GoalInput::simple(i));
    agda.agda.command(command).await?;
    if let Some(ctx) = preprint_agda_result(agda.agda.next_context().await?) {
        let ctx: Context = ctx;
        if ctx.context.is_empty() {
            println!("Context is empty, oops.");
        }
        for entry in ctx.context {
            if !entry.in_scope {
                print!("(Not in scope) ")
            }
            println!("{} : {}", entry.original_name, entry.binding);
        }
    }
    Ok(())
}

pub async fn ty(agda: &mut Repl, i: InteractionId) -> Monad {
    let command = Cmd::goal_type(GoalInput::simple(i));
    agda.agda.command(command).await?;
    if let Some(gs) = preprint_agda_result(agda.agda.next_goal_specific().await?) {
        match gs.goal_info {
            GoalInfo::CurrentGoal { the_type, .. } => println!("{}", the_type),
            _ => unreachable!(),
        }
    }
    Ok(())
}
