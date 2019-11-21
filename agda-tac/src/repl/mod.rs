use agda_mode::agda::ReplState;
use agda_mode::base::ComputeMode;
use agda_mode::cmd::Cmd;
use agda_mode::debug::{toggle_debug_command, toggle_debug_response};

use crate::file_io::{Monad, Repl};
use crate::input::{UserInput, HELP};
use crate::interact::help;

pub use self::goal::*;
pub use self::goal_list::*;
pub use self::line::*;

mod goal;
mod goal_list;
mod line;

pub async fn line(agda: &mut Repl, line: &str) -> Monad<bool> {
    line_impl(agda, UserInput::from(line)).await
}

async fn line_impl<'a>(agda: &mut Repl, line: UserInput<'a>) -> Monad<bool> {
    use UserInput::*;
    match line {
        Define(function_name) => define(agda, &function_name).await?,
        PushLine(code) => push_line(agda, code).await?,
        PopLine => pop_line(agda).await?,
        ShowLine(i) => show_line(agda, i),
        Give(i, new) => give(agda, i, new).await?,
        Infer(i, new) => infer(agda, i, new).await?,
        Simplify(i, new) => norm(agda, i, new, ComputeMode::DefaultCompute).await?,
        Normalize(i, new) => norm(agda, i, new, ComputeMode::UseShowInstance).await?,
        Type(i) => ty(agda, i).await?,
        Context(i) => ctx(agda, i).await?,
        Split(i, pat) => split(agda, i, pat).await?,
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
        ListGoals => agda.agda.print_goal_list(),
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
