use agda_mode::agda::ReplState;
use agda_mode::base::{debug_command, debug_response};
use std::fs::{File, create_dir, create_dir_all, remove_file};
use std::path::Path;

use crate::file_io::Repl;

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
const FAIL_RETRIEVE_HOME: &str = "Failed to retrieve home directory";

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
            println!("No input file specified.");
            let agda_tac_dir = dirs::home_dir()
                .expect(FAIL_RETRIEVE_HOME)
                .join(".agda-tac");
            let file_path = agda_tac_dir
                .join("Nameless.agda")
                .into_os_string()
                .into_string()
                .unwrap();
            println!("Default to {}", file_path);
            if Path::new(&file_path).exists() {
                remove_file(&file_path);
            }
            create_dir_all(agda_tac_dir);
            file_path
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
