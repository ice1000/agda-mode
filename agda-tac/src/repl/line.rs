use crate::file_io::{Monad, Repl};
use crate::repl::{reload_unit};
use agda_mode::pos::InteractionId;

pub fn show_line(agda: &mut Repl, i: usize) {
    let line_max = agda.line_count();
    if i >= line_max {
        eprintln!("There are only {} lines in total.", line_max);
    } else {
        print!("{}", agda.line_in_buffer(i))
    }
}

pub async fn pop_line(agda: &mut Repl) -> Monad {
    agda.remove_last_line()?;
    reload_unit(agda).await
}

pub async fn push_line(agda: &mut Repl, code: &str) -> Monad {
    agda.append(code)?;
    agda.append("\n")?;
    reload_unit(agda).await
}

pub async fn define(agda: &mut Repl, function_name: &&str) -> Monad {
    agda.append(&format!("{} : ?\n", function_name))?;
    agda.append(&format!("{} = ?\n", function_name))?;
    reload_unit(agda).await
}

pub async fn intro_pattern(agda: &mut Repl, i: InteractionId, new: &str) -> Monad {
    let ips = agda.agda.interaction_points();
    if i >= ips.len() as i32 || i < 0 {
        eprintln!("Bad interaction point: {:?}.", i);
        return Ok(());
    }
    let ip = ips[i as usize].clone();
    if agda.intros_in_goal_buffer(ip, new).is_none() {
        eprintln!("Don't know how to introduce, sorry.");
        Ok(())
    } else {
        agda.sync_buffer()?;
        reload_unit(agda).await
    }
}
