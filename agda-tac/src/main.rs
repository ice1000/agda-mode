use agda_mode::agda::ReplState;
use agda_mode::debug::{
    debug_command_via, debug_response_via, dont_debug_command, dont_debug_response,
};

use crate::file_io::{find_default_unwrap, InitModule, Repl};

/// Clap cli argument things.
mod args;
/// Rustyline completion & hints & things.
mod editor;
/// Buffer & file, for Agda interaction.
mod file_io;
/// Parse user input as a structural "command".
mod input;
/// Basic info about interaction, like `help`, read line & print things, etc.
mod interact;
/// Implementation of interaction.
mod repl;

const FAIL_WRITE: &str = "Failed to create Agda module file";
const FAIL: &str = "Failed to start Agda";
const FAIL_CMD: &str = "Failed to evaluate Agda command";

#[tokio::main]
async fn main() {
    let args = args::pre();
    unsafe {
        if args.debug_command {
            // Maybe we can do some fancy printing
            debug_command_via(|s| print!("{}", s))
        } else {
            dont_debug_command()
        }
        if args.debug_response {
            debug_response_via(|s| print!("{}", s))
        } else {
            dont_debug_response()
        }
    };
    let agda_program = args.agda.as_ref().map_or("agda", |s| &*s);
    let file = args.file.unwrap_or_else(find_default_unwrap);
    let InitModule(f, path, init) =
        file_io::init_module(file, args.allow_existing_file).expect(FAIL_WRITE);
    // Resolve path to an absolute PathBuf (canonical if possible)
    let abs_path = match std::fs::canonicalize(&path) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Failed to canonicalize path ({}): {:?}", path.display(), e);
            std::process::exit(1);
        }
    };
    let mut repl_state = ReplState::start(agda_program, abs_path).await.expect(FAIL);
    if args.validate {
        repl_state.validate_version_panicking().await;
        println!("It works!");
        std::process::exit(0);
    }
    let mut repl_state = Repl::new(repl_state, f, path, init);
    repl_state.is_plain = args.plain;
    interact::ion(repl_state).await.expect(FAIL_CMD);
}
