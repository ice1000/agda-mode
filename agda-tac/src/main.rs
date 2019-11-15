use crate::file_io::Repl;
use agda_mode::agda::ReplState;
use agda_mode::base::{debug_command, debug_response};

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
        debug_command(args.debug_command);
        debug_response(args.debug_response);
    };
    let agda_program = args.agda.as_ref().map_or("agda", |s| &*s);
    let file = match args.file {
        Some(file) => file,
        None => {
            eprintln!("No input file specified.");
            std::process::exit(1);
        }
    };
    let (f, path, first_line) = file_io::init_module(file).expect(FAIL_WRITE);
    let abs_path = match path.to_str() {
        None => {
            eprintln!("The given file name has some problems.");
            std::process::exit(1);
        }
        Some(f) => f.to_owned(),
    };
    let repl_state = ReplState::start(agda_program, abs_path).await.expect(FAIL);
    let mut repl_state = Repl::new(repl_state, f, path);
    repl_state.is_plain = args.plain;
    repl_state.append_line_buffer(first_line);
    interact::ion(repl_state).await.expect(FAIL_CMD);
}
