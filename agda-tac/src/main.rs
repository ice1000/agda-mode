use crate::repl::repl;
use agda_mode::agda::start_agda;
use tokio::io::BufReader;
use agda_mode::base::{debug_command, debug_response};

mod args;
mod repl;

#[tokio::main]
async fn main() {
    let args = args::pre();
    unsafe {
        debug_command(args.debug_command);
        debug_response(args.debug_response);
    };
    let agda_program = args.agda.as_ref().map_or("agda", |s| &*s);
    let (stdin, out) = start_agda(agda_program);
    (repl(stdin, BufReader::new(out), args.file).await).expect("Failed to evaluate Agda command");
}
