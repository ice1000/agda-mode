use std::io::{self, Write};

use rustyline::error::ReadlineError;

use crate::editor::CliEditor;
use crate::file_io::{history_file, Repl};
use crate::repl::{line, poll_goals};

const LAMBDA_LT: &str = "\u{03bb}> ";
const RICH_HELP: &str =
    "You're in the normal REPL, where there's completion, history command, hints and \
     (in the future) colored output.\n\
     The rich mode is not compatible with Windows PowerShell ISE and Mintty\
     (Cygwin, MinGW and (possibly, depends on your installation) git-bash).\n\
     If you're having problems with the rich mode, you may want to switch to \
     the plain mode (restart agda-tac with `--plain` flag).";
const PLAIN_HELP: &str = "You're in the plain REPL (with `--plain` flag).";

pub fn help(plain: bool) -> &'static str {
    if plain {
        PLAIN_HELP
    } else {
        RICH_HELP
    }
}

/// `interact::ion` stands for `interaction`.
pub async fn ion(mut agda: Repl) -> io::Result<()> {
    poll_goals(&mut agda.agda).await?;
    if agda.is_plain {
        let stdin = io::stdin();
        loop {
            print!("> ");
            io::stdout().flush()?;
            let mut next = String::new();
            stdin.read_line(&mut next)?;
            if line(&mut agda, next.trim()).await? {
                break;
            }
        }
    } else {
        let editor = CliEditor {};
        let mut r = editor.into_editor();
        let history_dir = history_file()?;
        if let Err(err) = r.load_history(&history_dir) {
            eprintln!("Failed to load REPL history: {:?}", err)
        }
        loop {
            match r.readline(LAMBDA_LT) {
                Ok(input) => {
                    let trim = input.trim();
                    r.add_history_entry(trim);
                    if line(&mut agda, trim).await? {
                        break;
                    }
                }
                Err(ReadlineError::Interrupted) => {}
                Err(ReadlineError::Eof) => break println!("Interrupted"),
                Err(err) => break eprintln!("Error: {:?}", err),
            }
        }
        if let Err(err) = r.save_history(&history_dir) {
            eprintln!("Failed to save REPL history: {:?}", err)
        }
    }
    Ok(())
}
