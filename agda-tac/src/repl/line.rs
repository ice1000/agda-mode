use crate::file_io::{Monad, Repl};
use crate::repl::reload;

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
    reload(agda).await?;
    Ok(())
}

pub async fn push_line(agda: &mut Repl, code: &str) -> Monad {
    agda.append(code)?;
    agda.append("\n")?;
    reload(agda).await?;
    Ok(())
}

pub async fn define(agda: &mut Repl, function_name: &&str) -> Monad {
    agda.append(&format!("{} : ?\n", function_name))?;
    agda.append(&format!("{} = ?\n", function_name))?;
    reload(agda).await?;
    Ok(())
}
