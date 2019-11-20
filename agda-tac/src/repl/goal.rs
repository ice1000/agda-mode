use agda_mode::agda::preprint_agda_result;
use agda_mode::base::ComputeMode;
use agda_mode::cmd::{Cmd, GoalInput};
use agda_mode::pos::InteractionId;
use agda_mode::resp::GoalInfo;

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
